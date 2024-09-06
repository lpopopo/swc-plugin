use core::fmt;

use swc_common::SyntaxContext;
use swc_common::{plugin::metadata::TransformPluginMetadataContextKind, Spanned};
use swc_core::atoms::Atom;
use swc_core::common::{Span, DUMMY_SP};
use swc_core::ecma::atoms::JsWord;
use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::*,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub struct TransformVisitor;

impl TransformVisitor {
    pub fn new() -> Self {
        TransformVisitor {}
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        call_expr.visit_mut_children_with(self);

        if let Callee::Expr(boxed_callee) = &mut call_expr.callee {
            if let Expr::Member(MemberExpr { obj, prop, .. }) = &mut **boxed_callee {
                if let MemberProp::Ident(IdentName { sym, .. }) = prop {
                    let last_then = find_last_then(call_expr);

                    if let Some(last_then) = last_then {
                        // Create the catch function
                        let catch_func = create_catch_arrow_func();

                        // Add the catch function to the end of the chain
                        let catch_expr = CallExpr {
                            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                                span: DUMMY_SP, // Adjust the span as needed
                                obj: Box::new(Expr::Call(CallExpr {
                                    callee: Callee::Expr(Box::new(Expr::Ident(
                                        Ident {
                                            span: DUMMY_SP,
                                            sym: "catch".into(),
                                            ctxt: SyntaxContext::empty(),
                                            optional: false,
                                        }
                                        .into(),
                                    ))),
                                    args: vec![catch_func.clone().into()],
                                    type_args: None,
                                    ctxt: SyntaxContext::empty(),
                                    span: DUMMY_SP,
                                })),
                                prop: MemberProp::Ident(IdentName {
                                    span: DUMMY_SP,
                                    sym: "catch".into(),
                                }),
                            }))),
                            args: vec![catch_func.clone().into()],
                            type_args: None,
                            ctxt: SyntaxContext::empty(),
                            span: DUMMY_SP,
                        };
                        *last_then = catch_expr;
                    }
                }
            }
        }
    }
}

fn find_last_then<'a>(call_expr: &'a mut CallExpr) -> Option<&'a mut CallExpr> {
    let mut current_expr: Option<&'a mut CallExpr> = Some(call_expr);
    let mut last_then_expr: Option<&'a mut CallExpr> = None;

    // 遍历找到最后的 then
    while let Some(expr) = current_expr {
        // 提取 callee 的不可变引用并判断是否是 then
        let is_then = {
            let callee = &mut expr.callee;
            if let Callee::Expr(boxed_callee) = callee {
                if let Expr::Member(MemberExpr { prop, .. }) = &**boxed_callee {
                    if let MemberProp::Ident(IdentName { sym, .. }) = prop {
                        sym == &JsWord::from("then")
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        };

        // 如果是 then 表达式
        if is_then {
            last_then_expr = Some(expr);

            let next_expr: Option<&'a mut CallExpr> = {
                if let Callee::Expr(boxed_callee) = &mut expr.callee {
                    if let Expr::Member(MemberExpr { obj, .. }) = &mut **boxed_callee {
                        if let Expr::Call(next_call) = &mut **obj {
                            Some(next_call)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            };

            if let Some(next_call) = next_expr {
                current_expr = Some(next_call);
                continue;
            }
        }

        // 如果没有下一个 then，结束循环
        current_expr = None;
    }

    // 返回最后找到的 then
    last_then_expr
}

fn create_catch_arrow_func() -> Expr {
    Expr::Arrow(ArrowExpr {
        params: vec![swc_core::ecma::ast::Pat::Ident(BindingIdent {
            id: Ident {
                span: DUMMY_SP,
                sym: "err".into(),
                optional: false,
                ctxt: SyntaxContext::empty(),
            },
            type_ann: None,
        })],
        body: Box::new(swc_core::ecma::ast::BlockStmtOrExpr::BlockStmt(BlockStmt {
            span: DUMMY_SP,
            stmts: vec![Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::Call(CallExpr {
                    callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                        span: DUMMY_SP,
                        sym: "console".into(),
                        optional: false,
                        ctxt: SyntaxContext::empty(),
                    }))),
                    args: vec![ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Ident(Ident {
                            span: DUMMY_SP,
                            sym: "err".into(),
                            optional: false,
                            ctxt: SyntaxContext::empty(),
                        })),
                    }],
                    type_args: None,
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                })),
            })],
            ctxt: SyntaxContext::empty(),
        })),
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
    })
}
/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

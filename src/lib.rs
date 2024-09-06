use core::fmt;
use std::borrow::{Borrow, BorrowMut};

use swc_common::SyntaxContext;
use swc_common::{plugin::metadata::TransformPluginMetadataContextKind, Spanned};
use swc_core::atoms::Atom;
use swc_core::common::{Span, DUMMY_SP};
use swc_core::ecma::ast;
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
                    if let Expr::Call(next_call_expr) = &mut **obj {
                        println!("next_call_expr   {:?}========>\n\n", next_call_expr);
                        if sym == "then" && !Some(next_call_expr).is_some() {
                            let catch_func = create_catch_arrow_func();

                            let catch_expr = CallExpr {
                                callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                                    span: DUMMY_SP,
                                    obj: Box::new(Expr::Call(call_expr.clone())),
                                    prop: MemberProp::Ident(IdentName {
                                        span: DUMMY_SP,
                                        sym: "catch".into(),
                                    }),
                                }))),
                                args: vec![catch_func.into()],
                                type_args: None,
                                span: DUMMY_SP,
                                ctxt: SyntaxContext::empty(),
                            };
                            *call_expr = catch_expr;
                        }
                    }
                }
            }
        }
    }
}

fn get_next_call_expr(call_expr: &mut CallExpr) -> Option<&mut CallExpr> {
    if let Callee::Expr(boxed_callee) = &mut call_expr.callee {
        if let Expr::Member(MemberExpr { obj, prop, .. }) = &mut **boxed_callee {
            if let MemberProp::Ident(IdentName { sym, .. }) = prop {
                if sym == "then" {
                    if let Expr::Call(next_call_expr) = &mut **obj {
                        return Some(next_call_expr);
                    }
                }
            }
        }
    }
    None
}

fn has_catch(call_expr: &mut CallExpr) -> bool {
    let mut current_expr = call_expr;
    while let Some(next_expr) = get_next_call_expr(current_expr) {
        if let Callee::Expr(boxed_callee) = &next_expr.callee {
            if let Expr::Member(MemberExpr { prop, .. }) = &**boxed_callee {
                if let MemberProp::Ident(IdentName { sym, .. }) = prop {
                    if sym == "catch" {
                        return true;
                    }
                }
            }
        }
        current_expr = next_expr;
    }
    false
}
fn create_catch_arrow_func() -> Expr {
    Expr::Arrow(ArrowExpr {
        params: vec![Pat::Ident(BindingIdent {
            id: Ident {
                span: DUMMY_SP,
                sym: "err".into(),
                optional: false,
                ctxt: SyntaxContext::empty(),
            },
            type_ann: None,
        })],
        body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
            span: DUMMY_SP,
            stmts: vec![Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::Call(CallExpr {
                    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: Box::new(Expr::Ident(Ident {
                            span: DUMMY_SP,
                            sym: "console".into(),
                            optional: false,
                            ctxt: SyntaxContext::empty(),
                        })),
                        prop: MemberProp::Ident(IdentName {
                            span: DUMMY_SP,
                            sym: "error".into(),
                        }),
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

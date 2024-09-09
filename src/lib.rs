use swc_common::SyntaxContext;
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

mod async_tool;
mod promise_tool;
use async_tool::{already_wrapped, wrap_arrow_body_with_try_catch, wrap_with_try_catch};
use promise_tool::{create_new_catch_callee, has_catch};

pub struct TransformVisitor;

impl TransformVisitor {
    pub fn new() -> Self {
        TransformVisitor {}
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        if let Callee::Expr(boxed_callee) = &mut call_expr.callee {
            if let Expr::Member(MemberExpr { prop, .. }) = &mut **boxed_callee {
                if let MemberProp::Ident(IdentName { sym, .. }) = prop {
                    if sym == "then" && !has_catch(call_expr) {
                        // Create the console.error(err) statement
                        create_new_catch_callee(call_expr);
                    }
                }
            }
        }
    }

    fn visit_mut_fn_decl(&mut self, node: &mut FnDecl) {
        // Check if options allow the transformation

        // If the function is async and it's not already wrapped, wrap it with try-catch
        if node.function.is_async {
            if let Some(body) = &mut node.function.body {
                if !already_wrapped(body) {
                    wrap_with_try_catch(body);
                }
            }
        }

        // Continue visiting the node
        node.visit_mut_children_with(self);
    }

    fn visit_mut_fn_expr(&mut self, node: &mut FnExpr) {
        // Check if options allow the transformation

        // If the function is async and it's not already wrapped, wrap it with try-catch
        if node.function.is_async {
            if let Some(body) = &mut node.function.body {
                if !already_wrapped(body) {
                    wrap_with_try_catch(body);
                }
            }
        }

        // Continue visiting the node
        node.visit_mut_children_with(self);
    }

    fn visit_mut_arrow_expr(&mut self, node: &mut ArrowExpr) {
        wrap_arrow_body_with_try_catch(node);
        node.visit_mut_children_with(self);
    }

    fn visit_mut_new_expr(&mut self, n: &mut NewExpr) {
        // 检查是否是 new Date()
        if let Expr::Ident(ident) = &*n.callee {
            if ident.sym == JsWord::new("Date") {
                // 检查第一个参数是否存在并且是字符串字面量
                if let Some(arg) = n.args.as_mut() {
                    if let Some(first_arg) = arg.first_mut() {
                        if let Expr::Lit(Lit::Str(Str { value, .. })) = &*first_arg.expr {
                            // 创建替换操作: "str".replace(/-/g, '/')
                            let replace_call = Expr::Call(CallExpr {
                                span: DUMMY_SP,
                                callee: Callee::Expr(Box::new(Expr::Member(
                                    swc_core::ecma::ast::MemberExpr {
                                        span: DUMMY_SP,
                                        obj: Box::new(Expr::Lit(Lit::Str(Str {
                                            value: value.clone(),
                                            span: DUMMY_SP,
                                            raw: None,
                                        }))),
                                        prop: MemberProp::Ident(IdentName::new(
                                            "replace".into(),
                                            DUMMY_SP,
                                        )),
                                    },
                                ))),
                                args: vec![
                                    ExprOrSpread {
                                        spread: None,
                                        expr: Box::new(Expr::Lit(Lit::Regex(Regex {
                                            exp: "-".into(),
                                            flags: "g".into(),
                                            span: DUMMY_SP,
                                        }))),
                                    },
                                    ExprOrSpread {
                                        spread: None,
                                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                                            value: "/".into(),
                                            span: DUMMY_SP,
                                            raw: None,
                                        }))),
                                    },
                                ],
                                type_args: None,
                                ctxt: SyntaxContext::empty(),
                            });

                            // 将第一个参数替换为 .replace(/-/g, '/')
                            first_arg.expr = Box::new(replace_call);
                        }
                    }
                }
            }
        }
        // 继续遍历其他节点
        n.visit_mut_children_with(self);
    }
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

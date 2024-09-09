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

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
mod new_date_tool;
mod promise_tool;
use async_tool::{already_wrapped, wrap_arrow_body_with_try_catch, wrap_with_try_catch};
use new_date_tool::create_new_regex_call;
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
                        create_new_catch_callee(call_expr);
                    }
                }
            }
        }
    }

    fn visit_mut_fn_decl(&mut self, node: &mut FnDecl) {
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
        if node.function.is_async {
            if let Some(body) = &mut node.function.body {
                if !already_wrapped(body) {
                    wrap_with_try_catch(body);
                }
            }
        }

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
                        if let Expr::Lit(Lit::Str(Str { value, .. })) = &mut *first_arg.expr {
                            let value_clone = value.clone();
                            create_new_regex_call(&mut first_arg.expr, value_clone);
                        }
                    }
                }
            }
        }
        // 继续遍历其他节点
        n.visit_mut_children_with(self);
    }

    fn visit_mut_assign_expr(&mut self, assign_expr: &mut AssignExpr) {
        // Get the replace operator (you'll need to implement push_cache)
        let replace_operator = push_assign_cache(&assign_expr.op);

        if replace_operator != "none" {
            let left_expr = match &assign_expr.left {
                AssignTarget::Simple(simple_target) => match simple_target {
                    SimpleAssignTarget::Ident(binding_ident) => {
                        Box::new(Expr::Ident(binding_ident.id.clone()))
                    }
                    SimpleAssignTarget::Member(member_expr) => {
                        Box::new(Expr::Member(member_expr.clone()))
                    }
                    SimpleAssignTarget::SuperProp(super_prop_expr) => {
                        Box::new(Expr::SuperProp(super_prop_expr.clone()))
                    }
                    SimpleAssignTarget::Paren(paren_expr) => {
                        Box::new(Expr::Paren(paren_expr.clone()))
                    }
                    SimpleAssignTarget::OptChain(opt_chain_expr) => {
                        Box::new(Expr::OptChain(opt_chain_expr.clone()))
                    }
                    SimpleAssignTarget::TsAs(ts_as_expr) => {
                        Box::new(Expr::TsAs(ts_as_expr.clone()))
                    }
                    SimpleAssignTarget::TsSatisfies(ts_satisfies_expr) => {
                        Box::new(Expr::TsSatisfies(ts_satisfies_expr.clone()))
                    }
                    SimpleAssignTarget::TsNonNull(ts_non_null_expr) => {
                        Box::new(Expr::TsNonNull(ts_non_null_expr.clone()))
                    }
                    SimpleAssignTarget::TsTypeAssertion(ts_type_assertion) => {
                        Box::new(Expr::TsTypeAssertion(ts_type_assertion.clone()))
                    }
                    SimpleAssignTarget::TsInstantiation(ts_instantiation) => {
                        Box::new(Expr::TsInstantiation(ts_instantiation.clone()))
                    }
                    SimpleAssignTarget::Invalid(_) => return, // 如果是无效的，我们不处理
                },
                AssignTarget::Pat(_) => return, // 如果是模式匹配，我们不处理
            };
            // Create the new call expression
            let new_right = Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                    replace_operator.into(),
                    DUMMY_SP,
                    SyntaxContext::empty(),
                )))),
                args: vec![
                    ExprOrSpread {
                        spread: None,
                        expr: left_expr,
                    },
                    ExprOrSpread {
                        spread: None,
                        expr: assign_expr.right.clone(),
                    },
                ],
                type_args: None,
                ctxt: SyntaxContext::empty(),
            });

            // Update the assignment expression
            assign_expr.right = Box::new(new_right);
            assign_expr.op = AssignOp::Assign;
        }

        // Continue visiting the node
        assign_expr.visit_mut_children_with(self);
    }

    fn visit_mut_bin_expr(&mut self, e: &mut BinExpr) {
        println!("Visiting BinExpr {:?}\n\n", e);
        e.visit_mut_children_with(self);

        let replace_operator = push_bin_cache(&e.op);
        if replace_operator != "none" {
            let call_expr = Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                    span: DUMMY_SP,
                    sym: replace_operator.into(),
                    optional: false,
                    ctxt: SyntaxContext::empty(),
                }))),
                args: vec![
                    ExprOrSpread {
                        spread: None,
                        expr: Box::new(*e.left.take()),
                    },
                    ExprOrSpread {
                        spread: None,
                        expr: Box::new(*e.right.take()),
                    },
                ],
                type_args: None,
                ctxt: SyntaxContext::empty(),
            });
            *e = BinExpr {
                span: DUMMY_SP,
                op: BinaryOp::EqEq, // 这里设置一个默认值，实际中你可能需要根据情况设置
                left: Box::new(call_expr),
                right: Box::new(Expr::Lit(Lit::Bool(Bool {
                    span: DUMMY_SP,
                    value: true,
                }))),
            };
        }
    }
}

fn push_assign_cache(op: &AssignOp) -> &'static str {
    match op {
        AssignOp::AddAssign => "accAdd",
        AssignOp::SubAssign => "accSub",
        AssignOp::MulAssign => "accMul",
        AssignOp::DivAssign => "accDiv",
        _ => "none",
    }
}

fn push_bin_cache(op: &BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "accAdd",
        BinaryOp::Sub => "accSub",
        BinaryOp::Mul => "accMul",
        BinaryOp::Div => "accDiv",
        BinaryOp::EqEqEq => "accCong",
        _ => "none",
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

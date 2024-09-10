use swc_common::{SyntaxContext, DUMMY_SP};
use swc_ecma_ast::*;

pub fn create_new_bin_call(new_op_call: &str, bin_expr: &mut BinExpr) -> Expr {
    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
            new_op_call.into(),
            DUMMY_SP,
            SyntaxContext::empty(),
        )))),
        args: vec![
            ExprOrSpread {
                spread: None,
                expr: Box::new(*bin_expr.left.clone()),
            },
            ExprOrSpread {
                spread: None,
                expr: Box::new(*bin_expr.right.clone()),
            },
        ],
        type_args: None,
        ctxt: SyntaxContext::empty(),
    })
}

pub fn create_assign_expr(
    left_expr: AssignTarget,
    right_expr: Box<Expr>,
    assign_expr: &mut AssignExpr,
    op: &str,
) {
    let new_left_expr = match &left_expr {
        AssignTarget::Simple(simple_target) => match simple_target {
            SimpleAssignTarget::Ident(binding_ident) => {
                Box::new(Expr::Ident(binding_ident.id.clone()))
            }
            SimpleAssignTarget::Member(member_expr) => Box::new(Expr::Member(member_expr.clone())),
            SimpleAssignTarget::SuperProp(super_prop_expr) => {
                Box::new(Expr::SuperProp(super_prop_expr.clone()))
            }
            SimpleAssignTarget::Paren(paren_expr) => Box::new(Expr::Paren(paren_expr.clone())),
            SimpleAssignTarget::OptChain(opt_chain_expr) => {
                Box::new(Expr::OptChain(opt_chain_expr.clone()))
            }
            SimpleAssignTarget::TsAs(ts_as_expr) => Box::new(Expr::TsAs(ts_as_expr.clone())),
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
            SimpleAssignTarget::Invalid(_) => return,
        },
        AssignTarget::Pat(_) => return,
    };

    let new_right = Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
            op.into(),
            DUMMY_SP,
            SyntaxContext::empty(),
        )))),
        args: vec![
            ExprOrSpread {
                spread: None,
                expr: new_left_expr,
            },
            ExprOrSpread {
                spread: None,
                expr: right_expr.clone(),
            },
        ],
        type_args: None,
        ctxt: SyntaxContext::empty(),
    });
    // Update the assignment expression
    assign_expr.right = Box::new(new_right);
    assign_expr.op = AssignOp::Assign;
}

pub fn create_require_statement(cache: Vec<String>) -> Stmt {
    Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![VarDeclarator {
            span: DUMMY_SP,
            name: Pat::Object(ObjectPat {
                span: DUMMY_SP,
                props: cache
                    .iter()
                    .map(|x| {
                        ObjectPatProp::Assign(AssignPatProp {
                            span: DUMMY_SP,
                            key: BindingIdent {
                                id: Ident::new(x.clone().into(), DUMMY_SP, SyntaxContext::empty()),
                                type_ann: None,
                            },
                            value: None,
                        })
                    })
                    .collect(),
                optional: false,
                type_ann: None,
            }),
            init: Some(Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                    "require".into(),
                    DUMMY_SP,
                    SyntaxContext::empty(),
                )))),
                args: vec![ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: "swc-plugin-accuracy/src/calc.js".into(),
                        raw: None,
                    }))),
                }],
                type_args: None,
                ctxt: SyntaxContext::empty(),
            }))),
            definite: false,
        }],
        ctxt: SyntaxContext::empty(),
    })))
}

pub fn push_assign_cache(op: &AssignOp) -> &'static str {
    match op {
        AssignOp::AddAssign => "accAdd",
        AssignOp::SubAssign => "accSub",
        AssignOp::MulAssign => "accMul",
        AssignOp::DivAssign => "accDiv",
        _ => "None",
    }
}

pub fn push_bin_cache(op: &BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "accAdd",
        BinaryOp::Sub => "accSub",
        BinaryOp::Mul => "accMul",
        BinaryOp::Div => "accDiv",
        BinaryOp::EqEqEq => "accCong",
        _ => "None",
    }
}

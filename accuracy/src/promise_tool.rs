use swc_common::{SyntaxContext, DUMMY_SP};
use swc_ecma_ast::*;

pub fn has_catch(call_expr: &mut CallExpr) -> bool {
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

pub fn get_next_call_expr(call_expr: &mut CallExpr) -> Option<&mut CallExpr> {
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

pub fn create_new_catch_callee(call_expr: &mut CallExpr) {
    let console_error_stmt = Stmt::Expr(ExprStmt {
        expr: Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident::new(
                    "console".into(),
                    DUMMY_SP,
                    SyntaxContext::empty(),
                ))),
                prop: MemberProp::Ident(IdentName::new("error".into(), DUMMY_SP)),
            }))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Ident(Ident::new(
                    "err".into(),
                    DUMMY_SP,
                    SyntaxContext::empty(),
                ))),
            }],
            type_args: None,
            ctxt: SyntaxContext::empty(),
        })),
        span: DUMMY_SP,
    });

    // Create the arrow function
    let arrow_func = Expr::Arrow(ArrowExpr {
        span: DUMMY_SP,
        params: vec![Pat::Ident(BindingIdent::from(Ident::new(
            "err".into(),
            DUMMY_SP,
            SyntaxContext::empty(),
        )))],
        body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
            span: DUMMY_SP,
            stmts: vec![console_error_stmt],
            ctxt: SyntaxContext::empty(),
        })),
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
        ctxt: SyntaxContext::empty(),
    });

    // Create the new function call with .catch
    let new_func = CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Call(call_expr.clone())),
            prop: MemberProp::Ident(IdentName::new("catch".into(), DUMMY_SP)),
        }))),
        args: vec![ExprOrSpread {
            spread: None,
            expr: Box::new(arrow_func),
        }],
        type_args: None,
        ctxt: SyntaxContext::empty(),
    };

    *call_expr = new_func;
}

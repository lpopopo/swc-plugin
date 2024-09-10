use swc_common::{SyntaxContext, DUMMY_SP};
use swc_ecma_ast::*;

// 帮助函数，检查函数体是否已被包裹
pub fn already_wrapped(block: &BlockStmt) -> bool {
    // 检查 block 中是否存在语句，并且第一个语句是否是 TryStmt
    if let Some(first_stmt) = block.stmts.first() {
        if let Stmt::Try(_) = first_stmt {
            // 如果第一个语句是 TryStmt 类型，则返回 true
            return true;
        }
    }

    // 如果没有找到 TryStmt，返回 false
    false
}

pub fn wrap_arrow_body_with_try_catch(node: &mut ArrowExpr) {
    if node.is_async {
        match &mut *node.body {
            // 如果函数体是块状，检查并包裹
            BlockStmtOrExpr::BlockStmt(block) => {
                if !already_wrapped(block) {
                    wrap_with_try_catch(block);
                }
            }
            // 如果函数体是表达式，转换为块并包裹
            BlockStmtOrExpr::Expr(expr) => {
                let block = BlockStmt {
                    span: DUMMY_SP,
                    stmts: vec![Stmt::Return(ReturnStmt {
                        span: DUMMY_SP,
                        arg: Some(expr.clone()),
                    })],
                    ctxt: SyntaxContext::empty(),
                };
                let mut wrapped_block = block.clone();
                wrap_with_try_catch(&mut wrapped_block);
                *node.body = *Box::new(BlockStmtOrExpr::BlockStmt(wrapped_block));
            }
        }
    }
}
// 帮助函数，用于将函数体包裹在 try-catch 中
pub fn wrap_with_try_catch(body: &mut BlockStmt) {
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
            args: vec![
                ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Ident(Ident::new(
                        "this".into(),
                        DUMMY_SP,
                        SyntaxContext::empty(),
                    ))),
                },
                ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Ident(Ident::new(
                        "error".into(),
                        DUMMY_SP,
                        SyntaxContext::empty(),
                    ))),
                },
            ],
            type_args: None,
            ctxt: SyntaxContext::empty(),
        })),
        span: DUMMY_SP,
    });
    let try_stmt = Stmt::Try(Box::new(TryStmt {
        block: body.clone(),
        handler: Some(CatchClause {
            param: Some(Pat::Ident(BindingIdent::from(Ident::new(
                "error".into(),
                DUMMY_SP,
                SyntaxContext::empty(),
            )))),
            body: BlockStmt {
                span: DUMMY_SP,
                stmts: vec![console_error_stmt],
                ctxt: SyntaxContext::empty(), // 如有需要，可以在此处添加自定义的 catch 逻辑
            },
            span: DUMMY_SP,
        }),
        finalizer: None,
        span: DUMMY_SP,
    }));

    // 将原始函数体替换为 try-catch 代码块
    body.stmts = vec![try_stmt];
}

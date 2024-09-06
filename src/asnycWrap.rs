use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Fold;
use swc_core::ecma::visit::FoldWith;

pub struct Wrap;

impl Fold for Wrap {
    fn fold_block_stmt(&mut self, block: BlockStmt) -> BlockStmt {
        // Your node (block) is now a BlockStmt in SWC's AST
        let try_stmt = TryStmt {
            block: BlockStmt {
                stmts: block.stmts,
                span: Default::default(), // You should handle proper spans
            },
            handler: Some(CatchClause {
                param: Some(Pat::Ident(BindingIdent {
                    id: Ident::new("error".into(), Default::default()),
                    type_ann: None,
                })),
                body: BlockStmt {
                    stmts: vec![Stmt::Expr(ExprStmt {
                        expr: Box::new(Expr::Call(CallExpr {
                            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                                obj: ExprOrSuper::Expr(Box::new(Expr::Ident(Ident::new(
                                    "console".into(),
                                    Default::default(),
                                )))),
                                prop: Box::new(Expr::Ident(Ident::new(
                                    "error".into(),
                                    Default::default(),
                                ))),
                                computed: false,
                                span: Default::default(),
                            }))),
                            args: vec![
                                ExprOrSpread {
                                    expr: Box::new(Expr::This(ThisExpr {
                                        span: Default::default(),
                                    })),
                                    spread: None,
                                },
                                ExprOrSpread {
                                    expr: Box::new(Expr::Ident(Ident::new(
                                        "error".into(),
                                        Default::default(),
                                    ))),
                                    spread: None,
                                },
                            ],
                            span: Default::default(),
                            type_args: None,
                        })),
                        span: Default::default(),
                    })],
                    span: Default::default(),
                },
                span: Default::default(),
            }),
            finalizer: None,
            span: Default::default(),
        };

        BlockStmt {
            stmts: vec![Stmt::Try(try_stmt)],
            span: block.span,
        }
    }
}

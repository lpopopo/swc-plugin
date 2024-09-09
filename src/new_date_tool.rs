use swc_common::{SyntaxContext, DUMMY_SP};
use swc_core::atoms::Atom;
use swc_ecma_ast::*;

pub fn create_new_regex_call(expr: &mut Box<Expr>, value: Atom) {
    // 创建替换操作: "str".replace(/-/g, '/')
    let replace_call = Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(Expr::Member(swc_core::ecma::ast::MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Lit(Lit::Str(Str {
                value: value.clone(),
                span: DUMMY_SP,
                raw: None,
            }))),
            prop: MemberProp::Ident(IdentName::new("replace".into(), DUMMY_SP)),
        }))),
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
    *expr = Box::new(replace_call);
}

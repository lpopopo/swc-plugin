use std::vec;
use swc_core::ecma::atoms::JsWord;
use swc_core::{
    ecma::{
        ast::*,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

mod async_tool;
mod config;
mod new_date_tool;
mod opration_tool;
mod promise_tool;
use async_tool::{already_wrapped, wrap_arrow_body_with_try_catch, wrap_with_try_catch};
use config::{parse_config, Config};
use new_date_tool::create_new_regex_call;
use opration_tool::{
    create_assign_expr, create_new_bin_call, create_require_statement, push_assign_cache,
    push_bin_cache,
};
use promise_tool::{create_new_catch_callee, has_catch};

pub struct TransformVisitor {
    pub cache: Vec<String>,

    pub parse_config: Config,

    pub has_polyfill_tag: bool,
}

impl TransformVisitor {
    pub fn new() -> Self {
        TransformVisitor {
            cache: vec![],
            has_polyfill_tag: false,
            parse_config: Config::new(true, true, true),
        }
    }

    fn cache_push(&mut self, cache: String) {
        if !self.cache.contains(&cache) {
            self.cache.push(cache)
        }
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_program(&mut self, program: &mut Program) {
        /*
         * 判断当前文件开头是否存在'calc polyfill'
         * 是则不处理这个文件
         */
        if let Program::Module(module) = program {
            if let Some(ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. }))) = module.body.get(0) {
                if let Expr::Lit(Lit::Str(Str { value, .. })) = &**expr {
                    if value == "calc polyfill" {
                        return;
                    }
                }
            }
        }
        program.visit_mut_children_with(self);
        if self.cache.len() > 0 {
            let new_stmt = create_require_statement(self.cache.clone());
            if let Program::Module(module) = program {
                module.body.insert(0, ModuleItem::Stmt(new_stmt));
            }
        }
    }

    // fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
    //     if let Callee::Expr(boxed_callee) = &mut call_expr.callee {
    //         if let Expr::Member(MemberExpr { prop, .. }) = &mut **boxed_callee {
    //             if let MemberProp::Ident(IdentName { sym, .. }) = prop {
    //                 if sym == "then" && !has_catch(call_expr) && self.parse_config.promise_catch {
    //                     create_new_catch_callee(call_expr);
    //                     return;
    //                 }
    //             }
    //         }
    //     }
    //     call_expr.visit_mut_children_with(self);
    // }

    fn visit_mut_fn_decl(&mut self, node: &mut FnDecl) {
        if node.function.is_async {
            if let Some(body) = &mut node.function.body {
                if !already_wrapped(body) && self.parse_config.add_async_try {
                    wrap_with_try_catch(body);
                }
            }
        }
        node.visit_mut_children_with(self);
    }

    fn visit_mut_fn_expr(&mut self, node: &mut FnExpr) {
        if !self.parse_config.add_async_try {
            return;
        }
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
        if !self.parse_config.add_async_try {
            return;
        }
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
        let replace_operator = push_assign_cache(&assign_expr.op);

        if replace_operator != "None" {
            if !self.parse_config.check_chong && replace_operator == "accCong" {
                return;
            }
            self.cache_push((*replace_operator).to_string());
            create_assign_expr(
                assign_expr.left.clone(),
                assign_expr.right.clone(),
                assign_expr,
                &replace_operator,
            );
        }

        // Continue visiting the node
        assign_expr.visit_mut_children_with(self);
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        if let Expr::Bin(bin_expr) = expr {
            let op = bin_expr.op;
            let new_op_call = push_bin_cache(&op);
            if new_op_call != "None" {
                if !self.parse_config.check_chong && new_op_call == "accCong" {
                    return;
                }
                self.cache_push((&new_op_call).to_string());
                // 创建一个函数调用表达式来替换二元表达式
                let new_expr = create_new_bin_call(new_op_call, bin_expr);
                // 替换原有的二元表达式
                *expr = new_expr;
            }
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    let parse_config = parse_config(
        &_metadata
            .get_transform_plugin_config()
            .expect("load plugin config failed"),
    );
    program.fold_with(&mut as_folder(TransformVisitor {
        cache: vec![],
        has_polyfill_tag: false,
        parse_config: Config {
            add_async_try: parse_config.add_async_try,
            promise_catch: parse_config.promise_catch,
            check_chong: parse_config.check_chong,
        },
    }))
}

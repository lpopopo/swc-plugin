use glob::glob;
use swc_common::plugin::metadata::TransformPluginMetadataContextKind;
use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::*,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
mod config;
pub use config::{parse_config, Config, ConfigFile};

pub struct TransformVisitor {
    pub config: Config,
    pub file_name: Option<String>,
}

impl TransformVisitor {
    pub fn new(config: Config, file_name: Option<String>) -> Self {
        TransformVisitor { config, file_name }
    }
}

fn stmt_delete_console(n: &mut Stmt, config: Config) {
    if let Stmt::Expr(expr_stmt) = n {
        if let Expr::Call(call) = &*expr_stmt.expr {
            if let Callee::Expr(callee_expr) = &call.callee {
                if let Expr::Member(member) = &**callee_expr {
                    if let Expr::Ident(indet) = &*member.obj {
                        print!("Stmt Ident is {}\n", indet);
                        if indet.sym.eq("console".into()) {
                            print!("delete console config is {:?}\n", config);
                            if let MemberProp::Ident(m_ident) = &member.prop {
                                if config.includes().contains(&m_ident.sym) {
                                    n.take();
                                } else if !config.excludes().contains(&m_ident.sym) {
                                    n.take();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    fn visit_mut_stmt(&mut self, n: &mut Stmt) {
        n.visit_mut_children_with(self);

        for file_include_rule in self.config.file().includes.clone() {
            for path in glob(&file_include_rule).unwrap() {
                print!("file config is {:?}\n", path);
                match path {
                    Ok(path) => {
                        if let Some(name) = self.file_name.clone() {
                            if name.contains(path.to_str().unwrap()) {
                                stmt_delete_console(n, self.config.clone());
                            }
                        }
                    }
                    Err(e) => println!("delete console plugin file include error {:?}", e),
                }
            }
        }
    }

    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        stmts.visit_mut_children_with(self);

        // We do same thing here.
        stmts.retain(|s| !matches!(s, Stmt::Empty(..)));
    }

    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);

        // This is also required, because top-level statements are stored in
        // `Vec<ModuleItem>`.
        stmts.retain(|s| {
            // We use `matches` macro as this match is trivial.
            !matches!(s, ModuleItem::Stmt(Stmt::Empty(..)))
        });
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
    let config = parse_config(
        &_metadata
            .get_transform_plugin_config()
            .expect("load plugin config failed"),
    );
    let file_name = _metadata.get_context(&TransformPluginMetadataContextKind::Filename);
    println!("file_name is {:?}", file_name);
    program.fold_with(&mut as_folder(TransformVisitor::new(config, file_name)))
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with
// mocks unless explicitly required to do so.
// test!(
//     Default::default(),
//     |_| as_folder(TransformVisitor),
//     boo,
//     // Input codes
//     r#"foo === bar;"#,
//     // Output codes after transformed with plugin
//     r#"kdy1 === bar;"#
// );

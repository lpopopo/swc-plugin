use std::path::PathBuf;

use delete_console::{Config, ConfigFile, TransformVisitor};
use swc_core::ecma::{transforms::testing::test_fixture, visit::as_folder};

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        Default::default(),
        &|_t| {
            as_folder(TransformVisitor::new(
                Config::new(
                    vec![],
                    vec!["warn".into()],
                    ConfigFile::new(vec!["**/tests/**/*.js".into()], vec![]),
                ),
                Option::Some((*input.to_str().unwrap()).to_string()),
            ))
        },
        &input,
        &output,
        Default::default(),
    );
}

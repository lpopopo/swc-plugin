use std::path::PathBuf;

use swc_core::ecma::{transforms::testing::test_fixture, visit::as_folder};
use swc_plugin_test::TransformVisitor;

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|t| as_folder(TransformVisitor),
        &input,
        &output,
        Default::default(),
    );
}

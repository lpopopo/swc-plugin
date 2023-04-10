use delete_console::{file_check, Config, ConfigFile};

#[test]
fn file_path_include() {
    let test_config = Config::new(
        vec![],
        vec![],
        ConfigFile::new(
            // vec!["**/tests/*.js".into()],
            vec![
                // "**/tests/**/*.js".into(),
                // "**/tests/*.js".into(),
                // "tests/**/*.js".into(),
                // "*/tests/**/*.js".into(),
                "**/tests/**/*.js".into(),
            ],
            vec![],
        ),
    );
    let test_file_path =
        "/Users/liushuai/Desktop/study/rust/swc_plugin/delete-console/tests/input.js";
    for rule_includes in test_config.file().includes.clone() {
        println!(
            "file check rule: {}   reslut------> {}",
            rule_includes,
            file_check(
                rule_includes.as_ref().to_string(),
                test_file_path.to_string()
            )
        )
    }
}

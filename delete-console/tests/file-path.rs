use glob::glob;

use delete_console::{Config, ConfigFile};

#[test]
fn file_path_include() {
    let test_config = Config::new(
        vec![],
        vec![],
        ConfigFile::new(vec!["**/tests/**/*.js".into()], vec![]),
    );
    for include_path in test_config.file().includes.clone() {
        for entry in glob(&include_path).unwrap() {
            match entry {
                Ok(entry) => {
                    println!("{:?}", entry);
                    println!(
                        "contains is {:?}",
                        include_path.contains(entry.to_str().unwrap())
                    );
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}

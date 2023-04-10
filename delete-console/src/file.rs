use std::path::{Component, PathBuf};

pub fn file_check(rule: String, path: String) -> bool {
    let binding = PathBuf::from(path.clone());
    let path_buf: Vec<_> = binding.components().collect();
    let rule_vec: Vec<&str> = rule.split('/').collect();
    //文件扩展名
    let file_extions = if let Some(last) = rule_vec.clone().pop() {
        if last.contains('.') {
            let file_extends_vec: Vec<&str> = last.split('.').collect();
            file_extends_vec.clone().pop()
        } else {
            Option::None
        }
    } else {
        Option::None
    };

    let mut path_index = 0;
    let mut rule_index = 0;
    let mut res = true;

    while rule_index < rule_vec.len() - 1 && path_index < path_buf.len() {
        let rule_parrent = rule_vec[rule_index];
        let path_dir = path_buf[path_index];
        if rule_parrent == "**" {
            let rule_component = Component::Normal(rule_vec[rule_index + 1].as_ref());
            if rule_component == path_dir {
                rule_index = rule_index + 2;
            }
            path_index = path_index + 1;
        } else {
            if rule_vec[rule_index] == path_dir.as_os_str() {
                rule_index = rule_index + 1;
            }
            res = false;
            break;
        }
    }
    if rule_index < rule_vec.len() {
        if let Some(extions) = file_extions {
            if path_index < path_buf.len() - 1 || !path.ends_with(extions) {
                res = false;
            }
        } else {
            if rule_vec[rule_vec.len() - 1] != path_buf[path_index].as_os_str() {
                res = false
            }
        }
    }
    res
}

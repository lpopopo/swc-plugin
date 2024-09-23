use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Display;

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    #[serde(default, rename = "checkChong")]
    pub check_chong: bool,
    #[serde(default, rename = "addAsyncTry")]
    pub add_async_try: bool,
    #[serde(default, rename = "promiseCatch")]
    pub promise_catch: bool,
}

impl Config {
    pub fn new(check_chong: bool, add_async_try: bool, promise_catch: bool) -> Config {
        Config {
            check_chong,
            add_async_try,
            promise_catch,
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "check_chong is {:?} add_async_try is {:?}  promise_catch is {:?}
            ",
            self.check_chong, self.add_async_try, self.promise_catch
        )
    }
}

pub fn parse_config(config_str: &str) -> Config {
    serde_json::from_str::<Config>(config_str).expect("Invalid plugin config")
}

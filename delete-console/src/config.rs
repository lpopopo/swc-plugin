use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Display;
use swc_core::ecma::atoms::JsWord;

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigFile {
    #[serde(default)]
    pub includes: Vec<JsWord>,
    #[serde(default)]
    pub excludes: Vec<JsWord>,
}

impl ConfigFile {
    pub fn new(includes: Vec<JsWord>, excludes: Vec<JsWord>) -> ConfigFile {
        ConfigFile { includes, excludes }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    includes: Vec<JsWord>,
    #[serde(default)]
    excludes: Vec<JsWord>,
    file: ConfigFile,
}

impl Config {
    pub fn new(includes: Vec<JsWord>, excludes: Vec<JsWord>, file: ConfigFile) -> Config {
        Config {
            includes,
            excludes,
            file,
        }
    }
    pub fn includes(&self) -> &[JsWord] {
        &self.includes
    }

    pub fn excludes(&self) -> &[JsWord] {
        &self.excludes
    }

    pub fn file(&self) -> &ConfigFile {
        &self.file
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}
            ",
            self.includes, self.excludes
        )
    }
}

pub fn parse_config(config_str: &str) -> Config {
    serde_json::from_str::<Config>(config_str).expect("Invalid plugin config")
}

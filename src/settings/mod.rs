use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

const DEFAULT_CFG: &str = ".tmlr/config";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Authentication {
    pub api_secret: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub auth: Option<Authentication>,
}

impl Settings {
    pub fn new(custom_path: Option<&str>) -> Result<Self, ConfigError> {
        let mut s = Config::new();
        if let Some(path) = custom_path {
            s.merge(File::with_name(path))?;
        } else {
            s.merge(File::with_name(&get_default_cfg_dir()))?;
        }
        // This makes it so "TMLR_AUTH__API_SECRET overrides the auth.api_secret key for example
        // the __ is used, because variables might be named some_var
        s.merge(Environment::with_prefix("tmlr").separator("__"))?;
        s.try_into()
    }
}

fn get_default_cfg_dir<'a>() -> String {
    match home::home_dir() {
        Some(path) => format!("{}/{}", path.display(), DEFAULT_CFG),
        None => DEFAULT_CFG.to_owned(),
    }
}

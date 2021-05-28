use crate::{
    error::Error::{ConfigurationError, DirCreationError, FileCreationError, FileWriteError},
    Result,
};
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File as StdFile},
    io::prelude::*,
};

const DEFAULT_FILE_NAME: &str = "config";
const DEFAULT_DIR_CFG: &str = ".tmlr";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Authentication {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub auth: Option<Authentication>,
}

impl Settings {
    pub fn new(custom_path: Option<&str>) -> Result<Self> {
        let mut s = Config::new();

        let path = match custom_path {
            Some(v) => v.to_owned(),
            None => get_default_file_path(),
        };
        s.merge(File::with_name(&path))
            .map_err(|e| ConfigurationError(e.to_string()))?;

        // This makes it so "TMLR_AUTH__API_SECRET overrides the auth.api_secret key for example
        // the __ is used, because variables might be named some_var
        s.merge(Environment::with_prefix("tmlr").separator("__"))
            .map_err(|e| ConfigurationError(e.to_string()))?;
        s.try_into().map_err(|e| ConfigurationError(e.to_string()))
    }

    pub fn save(&self, custom_path: Option<&str>) -> Result<()> {
        let cfg = toml::to_string(self)?;

        let path = match custom_path {
            Some(v) => v.to_owned(),
            None => {
                create_dir_all(get_default_cfg_dir())
                    .map_err(|e| DirCreationError(e.to_string()))?;
                get_default_file_path()
            }
        };

        let mut file = StdFile::create(&path).map_err(|e| FileCreationError(e.to_string()))?;
        file.write_all(cfg.as_bytes())
            .map_err(|e| FileWriteError(e.to_string()))?;

        Ok(())
    }
}

fn get_default_cfg_dir() -> String {
    match home::home_dir() {
        Some(path) => format!("{}/{}", path.display(), DEFAULT_DIR_CFG),
        None => DEFAULT_DIR_CFG.to_owned(),
    }
}

fn get_default_file_path() -> String {
    format!("{}/{}", get_default_cfg_dir(), DEFAULT_FILE_NAME)
}

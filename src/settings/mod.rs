use config::{Config, Environment, File};
use error::ConfigurationError::{self, *};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, File as StdFile},
    io::prelude::*,
    path::Path,
    result::Result as StdResult,
};

pub mod error;

const DEFAULT_FILE_NAME: &str = "config.toml";
const DEFAULT_DIR_CFG: &str = ".tmlr";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Authentication {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Alias {
    pub activity: Option<HashMap<String, String>>,
    pub tag: Option<HashMap<String, String>>,
}

impl Alias {
    pub fn add_activity_alias(&mut self, alias: String, activity_id: String) {
        let mut activity = match &self.activity {
            Some(v) => v.clone(),
            None => HashMap::with_capacity(1),
        };
        activity.insert(alias.to_lowercase(), activity_id);
        self.activity = Some(activity);
    }

    pub fn remove_activity_alias(&mut self, alias: &str) {
        if let Some(a) = &self.activity {
            let mut activity = a.clone();
            activity.remove(&alias.to_lowercase());
            if activity.is_empty() {
                self.activity = None;
            } else {
                self.activity = Some(activity);
            }
        }
    }

    pub fn add_tag_alias(&mut self, alias: String, tag_id: String) {
        let mut tag = match &self.tag {
            Some(v) => v.clone(),
            None => HashMap::with_capacity(1),
        };
        tag.insert(alias.to_lowercase(), tag_id);
        self.tag = Some(tag);
    }

    pub fn remove_tag_alias(&mut self, alias: &str) {
        if let Some(a) = &self.tag {
            let mut tag = a.clone();
            tag.remove(&alias.to_lowercase());
            if tag.is_empty() {
                self.tag = None;
            } else {
                self.tag = Some(tag);
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub auth: Option<Authentication>,
    pub alias: Option<Alias>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            auth: None,
            alias: None,
        }
    }
}

impl Settings {
    pub fn new(custom_path: Option<&str>) -> StdResult<Self, ConfigurationError> {
        let mut s = Config::new();

        let path = match custom_path {
            Some(v) => v.to_owned(),
            None => get_default_file_path(),
        };

        if !Path::new(&path).exists() {
            return Err(FileNotFoundError(path.to_owned()));
        }

        s.merge(File::with_name(&path))
            .map_err(|e| MessageError(e.to_string()))?;

        // This makes it so "TMLR_AUTH__API_SECRET overrides the auth.api_secret key for example
        // the __ is used, because variables might be named some_var
        s.merge(Environment::with_prefix("tmlr").separator("__"))
            .map_err(|e| MessageError(e.to_string()))?;
        s.try_into().map_err(|e| MessageError(e.to_string()))
    }

    pub fn save(&self, custom_path: Option<&str>) -> StdResult<(), ConfigurationError> {
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

    pub fn add_activity_alias(&mut self, alias: String, activity_id: String) {
        let mut alias_obj = self.alias.get_or_insert(Alias {
            activity: None,
            tag: None,
        });

        alias_obj.add_activity_alias(alias, activity_id);
    }

    pub fn add_tag_alias(&mut self, alias: String, tag_id: String) {
        let mut alias_obj = self.alias.get_or_insert(Alias {
            activity: None,
            tag: None,
        });

        alias_obj.add_tag_alias(alias, tag_id);
    }

    pub fn remove_activity_alias(&mut self, alias: &str) {
        if let Some(mut alias_obj) = self.alias.clone() {
            alias_obj.remove_activity_alias(alias);
            if alias_obj.tag.is_none() && alias_obj.activity.is_none() {
                self.alias = None
            } else {
                self.alias = Some(alias_obj)
            }
        }
    }

    pub fn remove_tag_alias(&mut self, alias: &str) {
        if let Some(mut alias_obj) = self.alias.clone() {
            alias_obj.remove_tag_alias(alias);
            if alias_obj.tag.is_none() && alias_obj.activity.is_none() {
                self.alias = None
            } else {
                self.alias = Some(alias_obj)
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_add_activity_alias() {
        let mut alias = Alias {
            activity: None,
            tag: None,
        };

        alias.add_activity_alias("a".to_owned(), "Some Activity".to_owned());
        let mut result = HashMap::with_capacity(2);
        result.insert("a", "Some Activity");
        assert!(matches!(
            alias,
            Alias {
                activity: Some(ref result),
                tag: None
            }
        ));

        alias.add_activity_alias("b".to_owned(), "Some Other Activity".to_owned());
        result.insert("b", "Some Other Activity");
        assert!(matches!(
            alias,
            Alias {
                activity: Some(result),
                tag: None
            }
        ));
    }

    #[test]
    fn test_alias_add_tag_alias() {
        let mut alias = Alias {
            activity: None,
            tag: None,
        };

        alias.add_tag_alias("x".to_owned(), "Tag x".to_owned());
        let mut result = HashMap::with_capacity(2);
        result.insert("x", "Tag x");
        assert!(matches!(
            alias,
            Alias {
                activity: None,
                tag: Some(ref result)
            }
        ));

        alias.add_tag_alias("y".to_owned(), "Tag y".to_owned());
        result.insert("y", "Tag y");
        assert!(matches!(
            alias,
            Alias {
                activity: None,
                tag: Some(result)
            }
        ));
    }

    #[test]
    fn test_alias_remove_tag_alias() {
        let mut tag_map = HashMap::with_capacity(2);
        tag_map.insert("x".to_owned(), "Tag x".to_owned());
        tag_map.insert("y".to_owned(), "Tag y".to_owned());

        let mut alias = Alias {
            activity: None,
            tag: Some(tag_map),
        };

        alias.remove_tag_alias("x");
        let mut result = HashMap::with_capacity(1);
        result.insert("y".to_owned(), "Tag y".to_owned());

        assert!(matches!(
            alias,
            Alias {
                activity: None,
                tag: Some(ref result)
            }
        ));

        alias.remove_tag_alias("y");
        assert!(matches!(
            alias,
            Alias {
                activity: None,
                tag: None
            }
        ));
    }

    #[test]
    fn test_alias_remove_activity_alias() {
        let mut activity_map = HashMap::with_capacity(2);
        activity_map.insert("a".to_owned(), "Acitivy A".to_owned());
        activity_map.insert("b".to_owned(), "Activity B".to_owned());

        let mut alias = Alias {
            activity: Some(activity_map),
            tag: None,
        };

        alias.remove_activity_alias("b");
        let mut result = HashMap::with_capacity(1);
        result.insert("a".to_owned(), "Activity B".to_owned());

        assert!(matches!(
            alias,
            Alias {
                activity: Some(ref result),
                tag: None
            }
        ));

        alias.remove_activity_alias("a");
        assert!(matches!(
            alias,
            Alias {
                activity: None,
                tag: None
            }
        ));
    }

    #[test]
    fn test_settings_add_activity_alias() {
        let mut settings = Settings {
            auth: None,
            alias: None,
        };

        settings.add_activity_alias("a".to_owned(), "Activity A".to_owned());
        let mut result = HashMap::with_capacity(2);
        result.insert("a", "Activity A");
        assert!(matches!(
            settings,
            Settings{
                auth: None,
                alias: Some(Alias {
                    activity: Some(ref result),
                    tag: None
                })
            }
        ));

        settings.add_activity_alias("b".to_owned(), "Activity B".to_owned());
        result.insert("b", "Activity B");
        assert!(matches!(
            settings,
            Settings{
                auth: None,
                alias: Some(Alias {
                    activity: Some(ref result),
                    tag: None
                })
            }
        ));
    }

    #[test]
    fn test_settings_add_tag_alias() {
        let mut settings = Settings {
            auth: None,
            alias: None,
        };

        settings.add_tag_alias("x".to_owned(), "Tag X".to_owned());
        let mut result = HashMap::with_capacity(2);
        result.insert("x", "Tag X");
        assert!(matches!(
            settings,
            Settings{
                auth: None,
                alias: Some(Alias {
                    activity: None,
                    tag: Some(ref result)
                })
            }
        ));

        settings.add_tag_alias("y".to_owned(), "Tag Y".to_owned());
        result.insert("y", "Tag Y");
        assert!(matches!(
            settings,
            Settings{
                auth: None,
                alias: Some(Alias {
                    activity: None,
                    tag: Some(ref result)
                })
            }
        ));
    }

    #[test]
    fn test_settings_remove_tag_alias() {
        let mut tag_map = HashMap::with_capacity(2);
        tag_map.insert("x".to_owned(), "Tag x".to_owned());
        tag_map.insert("y".to_owned(), "Tag y".to_owned());

        let mut settings = Settings {
            auth: None,
            alias: Some(Alias {
                activity: None,
                tag: Some(tag_map),
            }),
        };

        settings.remove_tag_alias("x");
        let mut result = HashMap::with_capacity(1);
        result.insert("y".to_owned(), "Tag y".to_owned());

        assert!(matches!(
            settings,
            Settings{
                auth: None,
                alias: Some(Alias{
                    activity: None,
                    tag: Some(ref result)
                })
            }
        ));

        settings.remove_tag_alias("y");
        assert!(matches!(
            settings,
            Settings {
                auth: None,
                alias: None
            }
        ));
    }

    #[test]
    fn test_settings_remove_activity_alias() {
        let mut activity_map = HashMap::with_capacity(2);
        activity_map.insert("a".to_owned(), "Acitivy A".to_owned());
        activity_map.insert("b".to_owned(), "Activity B".to_owned());

        let mut settings = Settings {
            auth: None,
            alias: Some(Alias {
                activity: Some(activity_map),
                tag: None,
            }),
        };

        settings.remove_activity_alias("b");
        let mut result = HashMap::with_capacity(1);
        result.insert("a".to_owned(), "Activity B".to_owned());

        assert!(matches!(
            settings,
            Settings{
                auth: None,
                alias: Some(Alias {
                    activity: Some(ref result),
                    tag: None
                })
            }
        ));

        settings.remove_activity_alias("a");
        assert!(matches!(
            settings,
            Settings {
                auth: None,
                alias: None
            }
        ));
    }
}

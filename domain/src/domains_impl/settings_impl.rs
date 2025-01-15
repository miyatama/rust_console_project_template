use crate::domains::settings::Settings;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use util::AppResult;
use util::Error::SettingInitializeError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingValue {
    pub todo_service_setting: TodoServiceSetting,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoServiceSetting {
    pub host: String,
    pub api_root: String,
    pub protocol: String,
}

#[derive(Clone)]
pub struct SettingsImpl {
    pub todo_service_setting: TodoServiceSetting,
}

impl SettingsImpl {
    pub fn new() -> AppResult<Self> {
        let setting_filename = "settings.yaml";
        let file = File::open(setting_filename);
        match file {
            Err(e) => {
                return Err(SettingInitializeError(format!("{:?}", e)));
            }
            _ => {}
        }
        let mut f = file.unwrap();
        let mut yaml = String::new();
        match f.read_to_string(&mut yaml) {
            Err(e) => {
                return Err(SettingInitializeError(format!("{:?}", e)));
            }
            _ => {}
        }
        let settings = serde_yml::from_str(&yaml);
        match settings {
            Err(e) => Err(SettingInitializeError(format!("{:?}", e))),
            Ok(settings) => {
                let settings: SettingValue = settings;
                Ok(Self {
                    todo_service_setting: settings.todo_service_setting.clone(),
                })
            }
        }
    }
}

impl Settings for SettingsImpl {
    fn get_todo_endpoint(&self) -> AppResult<String> {
        Ok(format!(
            "{}://{}{}",
            &self.todo_service_setting.protocol,
            &self.todo_service_setting.host,
            &self.todo_service_setting.api_root,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_todo_endpoint_success() {
        let settings_impl = SettingsImpl {
            todo_service_setting: TodoServiceSetting {
                host: "localhost:9999".to_string(),
                api_root: "/api".to_string(),
                protocol: "https".to_string(),
            },
        };
        let expect = "https://localhost:9999/api";
        let actual = settings_impl.get_todo_endpoint();
        assert_eq!(expect, actual.unwrap());
    }
}

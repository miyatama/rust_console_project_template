use crate::domains::settings::Settings;
use util::AppResult;

pub struct TodoServiceSetting {
    pub host: String,
    pub api_root: String,
    pub protocol: String,
}

pub struct SettingsImpl {
    pub todo_service_setting: TodoServiceSetting,
}

impl SettingsImpl {
    pub fn new() -> Self {
        // TODO yaml読み込み
        Self {
            todo_service_setting: TodoServiceSetting {
                host: "localhost:8080".to_string(),
                api_root: "/api".to_string(),
                protocol: "http".to_string(),
            },
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

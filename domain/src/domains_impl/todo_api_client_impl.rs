use crate::domains::settings::Settings;
use crate::domains::todo_api_client::TodoApiClient;
use crate::domains_impl::settings_impl::SettingsImpl;
use futures::executor;
use log::debug;
use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use util::AppResult;
use util::Error::TodoApiError;
use util::Todo;

pub struct TodoApiClientImpl {
    settings: SettingsImpl,
}

impl TodoApiClientImpl {
    pub fn new(settings: SettingsImpl) -> Self {
        Self { settings: settings }
    }
}

#[derive(Serialize, Deserialize)]
struct GetTodoParam {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct GetTodoResponse {
    todos: Vec<Todo>,
}

#[derive(Serialize, Deserialize)]
struct AddTodoParam {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateTodoParam {
    id: u32,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct DeleteTodoParam {
    id: u32,
}

impl TodoApiClient for TodoApiClientImpl {
    fn list(&self) -> AppResult<Vec<Todo>> {
        let future = async {
            let param = GetTodoParam {
                text: "".to_string(),
            };
            let endpoint = self.settings.get_todo_endpoint().unwrap();
            let url = format!("{}/todos", endpoint);
            debug!("get todos url: {}", &url);
            let param_string = serde_json::to_string(&param).unwrap();
            let client = reqwest::Client::new();
            let res = client
                .get(url)
                .header(CONTENT_TYPE, "application/json")
                .body(param_string)
                .send()
                .await;
            match res {
                Err(e) => {
                    return Err(TodoApiError(format!("{:?}", e)));
                }
                _ => {}
            }
            let res = res.unwrap();
            let content = res.text().await;
            match content {
                Err(e) => {
                    return Err(TodoApiError(format!("{:?}", e)));
                }
                _ => {}
            }
            let content = content.unwrap();
            debug!("get todo response: {}", &content);
            let response: GetTodoResponse = serde_json::from_str(&content).unwrap();
            Ok(response.todos)
        };
        executor::block_on(future)
    }

    fn create(&self, todo: Todo) -> AppResult<Todo> {
        let future = async move {
            let param = AddTodoParam { text: todo.text };
            let endpoint = self.settings.get_todo_endpoint().unwrap();
            let url = format!("{}/todo", endpoint);
            let param_string = serde_json::to_string(&param).unwrap();
            let client = reqwest::Client::new();
            let res = client
                .post(url)
                .header(CONTENT_TYPE, "application/json")
                .body(param_string)
                .send()
                .await;
            match res {
                Err(e) => {
                    return Err(TodoApiError(format!("{:?}", e)));
                }
                _ => {}
            }
            let res = res.unwrap();
            let content = res.text().await;
            match content {
                Err(e) => {
                    return Err(TodoApiError(format!("{:?}", e)));
                }
                _ => {}
            }
            let content = content.unwrap();
            debug!("create todo response: {}", &content);
            let response: Todo = serde_json::from_str(&content).unwrap();
            Ok(response)
        };
        executor::block_on(future)
    }

    fn update(&self, todo: Todo) -> AppResult<Todo> {
        let future = async move {
            let param = UpdateTodoParam {
                id: todo.id,
                text: todo.text,
            };
            let endpoint = self.settings.get_todo_endpoint().unwrap();
            let url = format!("{}/todo", endpoint);
            let param_string = serde_json::to_string(&param).unwrap();
            let client = reqwest::Client::new();
            let res = client
                .put(url)
                .header(CONTENT_TYPE, "application/json")
                .body(param_string)
                .send()
                .await;
            match res {
                Err(e) => {
                    return Err(TodoApiError(format!("{:?}", e)));
                }
                _ => {}
            }
            let res = res.unwrap();
            let content = res.text().await;
            match content {
                Err(e) => {
                    return Err(TodoApiError(format!("{:?}", e)));
                }
                _ => {}
            }
            let content = content.unwrap();
            debug!("update todo response: {}", &content);
            let response: Todo = serde_json::from_str(&content).unwrap();
            Ok(response)
        };
        executor::block_on(future)
    }

    fn delete(&self, todo: Todo) -> AppResult<()> {
        let future = async move {
            let param = DeleteTodoParam { id: todo.id };
            let endpoint = self.settings.get_todo_endpoint().unwrap();
            let url = format!("{}/todo", endpoint);
            let param_string = serde_json::to_string(&param).unwrap();
            let client = reqwest::Client::new();
            let res = client
                .delete(url)
                .header(CONTENT_TYPE, "application/json")
                .body(param_string)
                .send()
                .await;
            match res {
                Err(e) => {
                    return Err(TodoApiError(format!("{:?}", e)));
                }
                _ => {}
            }
            let _res = res.unwrap();
            Ok(())
        };
        executor::block_on(future)
    }
}

use util::AppResult;

#[cfg(feature = "mock")]
use mockall::automock;

#[cfg_attr(feature = "mock", automock)]
pub trait Settings {
    fn get_todo_endpoint(&self) -> AppResult<String>;
}

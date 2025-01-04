/*
cfg_if::cfg_if! {
    if #[cfg(feature = "mock")] {
        use crate::MockTodoApiClient as TodoApiClient;
    } else {
        use crate::TodoApiClient;
    }
}
*/
use domain::TodoApiClient;

// TODO DomainHandlerをautomockしたい。
// #[cfg(feature = "mock")]
// use mockall::automock;

// ref: https://docs.rs/mockall/latest/mockall/#associated-types
// #[cfg_attr(feature = "mock", automock)]
pub trait DomainHandler {
    type TodoApi: TodoApiClient;

    fn todo_api_client(&self) -> &Self::TodoApi;
}

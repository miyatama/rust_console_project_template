mod domains;
mod domains_impl;

// #[cfg_attr(feature = "mock", mockall_double::double)]
#[cfg(feature = "mock")]
pub use domains::todo_api_client::MockTodoApiClient;
pub use domains::todo_api_client::TodoApiClient;

pub use domains_impl::todo_api_client_impl::TodoApiClientImpl;

mod domains;
mod domains_impl;

// #[cfg(feature = "mock")]
// use mockall_double::double;

// #[cfg_attr(feature = "mock", double)]
#[cfg(feature = "mock")]
pub use domains::todo_api_client::MockTodoApiClient;
pub use domains::todo_api_client::TodoApiClient;

pub use domains_impl::todo_api_client_impl::TodoApiClientImpl;

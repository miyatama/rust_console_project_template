mod domains;
mod domains_impl;

#[cfg(feature = "mock")]
pub use domains::todo_api_client::MockTodoApiClient;
pub use domains::todo_api_client::TodoApiClient;

pub use domains_impl::todo_api_client_impl::TodoApiClientImpl;

#[cfg(feature = "mock")]
pub use domains::settings::MockSettings;
pub use domains::settings::Settings;
pub use domains_impl::settings_impl::SettingsImpl;

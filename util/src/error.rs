#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed reqwest request: {}", .0)]
    // TodoApiError(#[from] reqwest::Error),
    TodoApiError(String),
    #[error("{}", .0)]
    Unknown(#[from] anyhow::Error),
}

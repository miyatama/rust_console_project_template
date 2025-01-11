mod app_logger;
mod data;
mod error;

pub use app_logger::AppLogger;

pub use data::todo::Todo;

pub use error::Error;

pub type AppResult<T> = anyhow::Result<T, Error>;

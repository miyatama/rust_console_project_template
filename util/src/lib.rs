mod app_logger;
mod data;
mod error;

pub use app_logger::AppLogger;

pub use data::todo::Todo;

pub use error::Error;

use anyhow::Result;
pub type AppResult<T> = Result<T, Error>;

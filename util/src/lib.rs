mod data;
mod error;

pub use data::todo::Todo;

pub use error::Error;

pub type AppResult<T> = anyhow::Result<T, Error>;

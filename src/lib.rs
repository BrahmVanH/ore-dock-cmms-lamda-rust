pub mod error;
pub mod models;
pub mod schema;
pub mod db;

pub use error::{AppError, AppResult};
pub use models::prelude::*;
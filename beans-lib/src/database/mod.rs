//! Database module for SQLite persistence.

mod repository;
mod schema;
mod sqlite_repository;

pub use repository::{EntryFilter, Repository};
pub use schema::initialize_schema;
pub use sqlite_repository::SQLiteRepository;

mod common;
mod in_memory;

pub use common::Database;

pub fn get_database() -> impl Database {
    in_memory::InMemoryDatabase::new()
}

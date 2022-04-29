mod common;
mod in_memory;

pub use common::Database;
pub use in_memory::InMemoryDatabase;

pub fn get_database() -> InMemoryDatabase {
    InMemoryDatabase::new()
}

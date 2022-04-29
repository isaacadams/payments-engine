use crate::models::transaction::Transaction;
use crate::services::account_state::AccountState;

pub trait Database {
    fn add_transaction(&mut self, x: Transaction);
    fn fetch_client(&mut self, id: u16) -> &mut AccountState;
    fn print(&self);
}

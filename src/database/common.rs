use crate::models::{account::Account, transaction::Transaction};
use crate::services::account_state::AccountState;

pub trait Database {
    fn add_transaction(&mut self, x: Transaction);
    fn fetch_client_mut(&mut self, id: u16) -> &mut AccountState;
    fn fetch_client_ref(&mut self, id: u16) -> &AccountState;
    fn get_transaction_amt(&self, tx_id: u32) -> Option<f32>;
    fn get_transaction(&self, tx_id: u32) -> Option<&Transaction>;
    fn get_accounts(&self) -> Vec<Account>;
}

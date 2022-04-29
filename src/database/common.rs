use crate::models::account::Account;
use crate::services::{account_state::AccountState, transaction_state::TransactionState};

pub trait Database {
    fn add_transaction(&mut self, x: TransactionState);
    fn fetch_client_mut(&mut self, id: u16) -> &mut AccountState;
    fn fetch_client_ref(&mut self, id: u16) -> &AccountState;
    fn get_transaction(&self, tx_id: u32) -> Option<&TransactionState>;
    fn get_transaction_mut(&mut self, tx_id: u32) -> Option<&mut TransactionState>;
    fn get_accounts(&self) -> Vec<Account>;
}

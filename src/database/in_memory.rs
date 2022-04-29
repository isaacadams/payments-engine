use super::common::Database;
use crate::models::{account::Account, transaction::Transaction};
use crate::services::account_state::AccountState;
use std::collections::HashMap;

pub struct InMemoryDatabase {
    clients: HashMap<u16, AccountState>,
    transactions: HashMap<u32, Transaction>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        InMemoryDatabase {
            clients: HashMap::new(),
            transactions: HashMap::new(),
        }
    }
}

impl Database for InMemoryDatabase {
    fn add_transaction(&mut self, x: Transaction) {
        self.transactions.insert(x.tx_id, x);
    }

    fn fetch_client_mut(&mut self, id: u16) -> &mut AccountState {
        self.clients.entry(id).or_insert(AccountState::new(id))
    }

    fn fetch_client_ref(&mut self, id: u16) -> &AccountState {
        self.clients.entry(id).or_insert(AccountState::new(id))
    }

    fn get_transaction_amt(&self, tx_id: u32) -> Option<f32> {
        if let Some(txn) = self.transactions.get(&tx_id) {
            return txn.amt;
        }

        None
    }

    fn get_transaction(&self, tx_id: u32) -> Option<&Transaction> {
        if let Some(txn) = self.transactions.get(&tx_id) {
            return Some(txn);
        }

        None
    }

    fn get_accounts(&self) -> Vec<Account> {
        let accounts: Vec<Account> = self.clients.iter().map(|(_, state)| state.into()).collect();
        accounts
    }
}

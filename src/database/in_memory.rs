use super::common::Database;
use crate::models::account::Account;
use crate::services::{account_state::AccountState, transaction_state::TransactionState};
use std::collections::HashMap;

pub struct InMemoryDatabase {
    clients: HashMap<u16, AccountState>,
    transactions: HashMap<u32, TransactionState>,
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
    fn add_transaction(&mut self, x: TransactionState) {
        self.transactions.insert(x.tx_id, x);
    }

    fn fetch_client_mut(&mut self, id: u16) -> &mut AccountState {
        self.clients
            .entry(id)
            .or_insert_with(|| AccountState::new(id))
    }

    fn fetch_client_ref(&mut self, id: u16) -> &AccountState {
        self.clients
            .entry(id)
            .or_insert_with(|| AccountState::new(id))
    }

    fn get_transaction(&self, tx_id: u32) -> Option<&TransactionState> {
        if let Some(txn) = self.transactions.get(&tx_id) {
            return Some(txn);
        }

        None
    }

    fn get_transaction_mut(&mut self, tx_id: u32) -> Option<&mut TransactionState> {
        self.transactions.get_mut(&tx_id)
    }

    fn get_accounts(&self) -> Vec<Account> {
        let accounts: Vec<Account> = self.clients.iter().map(|(_, state)| state.into()).collect();
        accounts
    }
}

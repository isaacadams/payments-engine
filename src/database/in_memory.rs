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

    fn fetch_client(&mut self, id: u16) -> &mut AccountState {
        self.clients.entry(id).or_insert(AccountState::new(id))
    }

    fn print(&self) {
        let accounts: Vec<Account> = self
            .clients
            .iter()
            .map(|(_, state)| state.into())
            .collect();

        println!("{:?}", accounts);
    }
}

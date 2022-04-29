use crate::models::account::Account;

pub struct AccountState {
    pub id: u16,
    pub available: f32,
    pub held: f32,
    pub locked: bool,
}

impl AccountState {
    pub fn new(id: u16) -> Self {
        AccountState {
            id,
            available: 0_f32,
            held: 0_f32,
            locked: false,
        }
    }

    pub fn withdraw(&mut self, amount: f32) {
        if self.total() >= amount {
            self.available = &self.available - amount;
        }
    }

    pub fn deposit(&mut self, amount: f32) {
        self.available = &self.available + amount;
    }

    fn total(&self) -> f32 {
        self.available + self.held
    }
}

impl Into<Account> for &AccountState {
    fn into(self) -> Account {
        Account {
            client_id: self.id,
            available: self.available,
            held: self.held,
            total: self.total(),
            locked: self.locked,
        }
    }
}

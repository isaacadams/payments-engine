use crate::models::account::Account;

pub struct AccountState {
    id: u16,
    available: f32,
    held: f32,
    locked: bool,
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

    pub fn withdraw(&mut self, amount: f32) -> bool {
        if self.available >= amount {
            self.available -= amount;
            return true;
        }

        false
    }

    pub fn deposit(&mut self, amount: f32) {
        self.available += amount;
    }

    pub fn dispute(&mut self, amount: f32) {
        self.available -= amount;
        self.held += amount;
    }

    pub fn resolve(&mut self, amount: f32) {
        self.available += amount;
        self.held -= amount;
    }

    pub fn chargeback(&mut self, amount: f32) {
        self.held -= amount;
        self.locked = true;
    }

    fn total(&self) -> f32 {
        self.available + self.held
    }
}

impl From<&AccountState> for Account {
    fn from(state: &AccountState) -> Self {
        Account {
            client: state.id,
            available: state.available,
            held: state.held,
            total: state.total(),
            locked: state.locked,
        }
    }
}

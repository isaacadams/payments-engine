use crate::Transaction;

pub struct TransactionState {
    pub tx_id: u32,
    pub client_id: u16,
    pub amt: f32,
    disputed: bool,
}

impl TransactionState {
    pub fn dispute(&mut self) {
        self.disputed = true;
    }

    pub fn resolve(&mut self) -> bool {
        if self.disputed {
            self.disputed = false;
            return true;
        }

        false
    }
}

impl From<Transaction> for TransactionState {
    fn from(t: Transaction) -> Self {
        TransactionState {
            tx_id: t.tx_id,
            client_id: t.client_id,
            amt: t.get_amt(),
            disputed: false,
        }
    }
}

use super::models::transaction::{Transaction, TransactionType};
//use super::services::account_state::AccountState;
use super::Database;
use super::{PaymentEngineError, PaymentEngineResult};

pub struct TransactionHandler<T: Database> {
    database: T,
}

impl<T: Database> From<T> for TransactionHandler<T> {
    fn from(d: T) -> Self {
        TransactionHandler { database: d }
    }
}

impl<T: Database> TransactionHandler<T> {
    pub fn handle_transaction(&mut self, x: Transaction) -> PaymentEngineResult<()> {
        match x.tx_type {
            TransactionType::Withdrawal => {
                if let Some(amt) = x.amt {
                    let account = self.database.fetch_client_mut(x.client_id);
                    if account.withdraw(amt) {
                        self.database.add_transaction(x);
                    } else {
                        return Err(PaymentEngineError::NotEnoughFunds(x));
                    }
                } else {
                    return Err(PaymentEngineError::ExpectedAmount(x));
                }
            }
            TransactionType::Deposit => {
                if let Some(amt) = x.amt {
                    let account = self.database.fetch_client_mut(x.client_id);
                    account.deposit(amt);
                    self.database.add_transaction(x);
                } else {
                    return Err(PaymentEngineError::ExpectedAmount(x));
                }
            }
            TransactionType::Dispute => {
                if let Some(amt) = self.database.get_transaction_amt(x.tx_id) {
                    let account = self.database.fetch_client_mut(x.client_id);
                    account.dispute(amt);
                } else {
                    return Err(PaymentEngineError::ExpectedTransactionToExist(x));
                }
            }
            _ => (),
            /*
            TransactionType::Resolve => {},
            TransactionType::Chargeback => {},
            */
        };

        Ok(())
    }

    pub fn get_database(self) -> T {
        self.database
    }
}

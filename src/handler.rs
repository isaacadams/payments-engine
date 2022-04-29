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
    pub fn handle(&mut self, x: Transaction) -> PaymentEngineResult<()> {
        match x.tx_type {
            TransactionType::Withdrawal => {
                if let Some(amt) = x.amt {
                    let account = self.database.fetch_client_mut(x.client_id);
                    if account.withdraw(amt) {
                        self.database.add_transaction(x.into());
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
                    self.database.add_transaction(x.into());
                } else {
                    return Err(PaymentEngineError::ExpectedAmount(x));
                }
            }
            TransactionType::Dispute => {
                match self.get_transaction_details(x.tx_id) {
                    None => return Err(PaymentEngineError::ExpectedTransactionToExist(x)),
                    Some((amt, client_id)) => {
                        if x.client_id != client_id {
                            return Err(PaymentEngineError::ExpectedClientIdToMatch(x));
                        }

                        let account = self.database.fetch_client_mut(x.client_id);
                        account.dispute(amt);
                    }
                };
            }
            _ => (),
            /*
            TransactionType::Resolve => {},
            TransactionType::Chargeback => {},
            */
        };

        Ok(())
    }

    fn get_transaction_details(&self, id: u32) -> Option<(f32, u16)> {
        self.database
            .get_transaction(id)
            .map(|txn| (txn.amt, txn.client_id))
    }

    pub fn get_database(self) -> T {
        self.database
    }
}

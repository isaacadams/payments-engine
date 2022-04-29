use super::models::transaction::{Transaction, TransactionType};
//use super::services::account_state::AccountState;
use super::Database;
use super::{PaymentEngineError, PaymentEngineResult, TransactionHandlerError};

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
        let error = match x.tx_type {
            TransactionType::Withdrawal => self.withdraw(&x),
            TransactionType::Deposit => self.deposit(&x),
            TransactionType::Dispute => self.dispute(&x),
            TransactionType::Resolve => self.resolve(&x),
            _ => None,
            /*
            TransactionType::Chargeback => {},
            */
        };

        match error {
            Some(e) => return Err(PaymentEngineError::TransactionHandler(e, x)),
            None => {
                if x.tx_type == TransactionType::Withdrawal || x.tx_type == TransactionType::Deposit
                {
                    self.database.add_transaction(x.into());
                }
            }
        }

        Ok(())
    }

    fn withdraw(&mut self, x: &Transaction) -> Option<TransactionHandlerError> {
        match x.amt {
            Some(amt) => {
                let account = self.database.fetch_client_mut(x.client_id);
                if account.withdraw(amt) {
                    //self.database.add_transaction(x.into());
                    None
                } else {
                    Some(TransactionHandlerError::NotEnoughFunds)
                }
            }
            None => Some(TransactionHandlerError::ExpectedAmount),
        }
    }

    fn deposit(&mut self, x: &Transaction) -> Option<TransactionHandlerError> {
        match x.amt {
            Some(amt) => {
                let account = self.database.fetch_client_mut(x.client_id);
                account.deposit(amt);
                //self.database.add_transaction(x.into());
                None
            }
            None => Some(TransactionHandlerError::ExpectedAmount),
        }
    }

    fn dispute(&mut self, x: &Transaction) -> Option<TransactionHandlerError> {
        match self.database.get_transaction_mut(x.tx_id).map(|txn| {
            if x.client_id != txn.client_id {
                Err(TransactionHandlerError::ExpectedClientIdToMatch)
            } else {
                //if txn is already disputed, throw error
                txn.dispute();
                Ok(txn.amt)
            }
        }) {
            Some(data) => match data {
                Err(e) => Some(e),
                Ok(amt) => {
                    let account = self.database.fetch_client_mut(x.client_id);
                    account.dispute(amt);
                    None
                }
            },
            None => Some(TransactionHandlerError::ExpectedTransactionToExist),
        }
    }

    fn resolve(&mut self, x: &Transaction) -> Option<TransactionHandlerError> {
        self.database
            .get_transaction_mut(x.tx_id)
            .map(|txn| (txn.resolve(), txn.amt))
            .map_or(
                Some(TransactionHandlerError::ExpectedTransactionToExist),
                |(s, amt)| {
                    if s {
                        let account = self.database.fetch_client_mut(x.client_id);
                        account.resolve(amt);
                        None
                    } else {
                        Some(TransactionHandlerError::MustBeInActiveDispute)
                    }
                },
            )
    }

    pub fn get_database(self) -> T {
        self.database
    }
}

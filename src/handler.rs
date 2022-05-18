use super::models::transaction::{Transaction, TransactionType};
use super::services::{account_state::AccountState, transaction_state::TransactionState};
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
        let result = match x.tx_type {
            TransactionType::Withdrawal => self.withdraw(&x),
            TransactionType::Deposit => self.deposit(&x),
            TransactionType::Dispute => self.dispute(&x),
            TransactionType::Resolve => self.resolve(&x, |amt, a| {
                a.resolve(amt);
            }),
            TransactionType::Chargeback => self.resolve(&x, |amt, a| {
                a.chargeback(amt);
            }),
        };

        match result.err() {
            Some(e) => return Err(PaymentEngineError::TransactionHandler(e, x)),
            None => {
                if x.tx_type == TransactionType::Withdrawal || x.tx_type == TransactionType::Deposit
                {
                    self.database.add_transaction(x.into());
                }
            }
        };

        Ok(())
    }

    fn withdraw(&mut self, x: &Transaction) -> Result<(), TransactionHandlerError> {
        match x.amount {
            Some(amt) => {
                let account = self.database.fetch_client_mut(x.client_id);
                if account.withdraw(amt) {
                    Ok(())
                } else {
                    Err(TransactionHandlerError::NotEnoughFunds)
                }
            }
            None => Err(TransactionHandlerError::ExpectedAmount),
        }
    }

    fn deposit(&mut self, x: &Transaction) -> Result<(), TransactionHandlerError> {
        match x.amount {
            Some(amt) => {
                let account = self.database.fetch_client_mut(x.client_id);
                account.deposit(amt);
                Ok(())
            }
            None => Err(TransactionHandlerError::ExpectedAmount),
        }
    }

    fn dispute(&mut self, x: &Transaction) -> Result<(), TransactionHandlerError> {
        let (amt, client_id) = self.fetch_transaction(x.tx_id, x.client_id, |txn| {
            txn.dispute();
            (txn.amt, txn.client_id)
        })?;

        let account = self.database.fetch_client_mut(client_id);
        account.dispute(amt);
        Ok(())
    }

    fn resolve<F>(&mut self, x: &Transaction, f: F) -> Result<(), TransactionHandlerError>
    where
        F: FnOnce(f32, &mut AccountState),
    {
        let (s, amt, client_id) = self.fetch_transaction(x.tx_id, x.client_id, |txn| {
            (txn.resolve(), txn.amt, txn.client_id)
        })?;

        if !s {
            return Err(TransactionHandlerError::MustBeInActiveDispute);
        }

        let account = self.database.fetch_client_mut(client_id);
        f(amt, account);
        Ok(())
    }

    fn fetch_transaction<F, U>(
        &mut self,
        tx_id: u32,
        client_id: u16,
        f: F,
    ) -> Result<U, TransactionHandlerError>
    where
        F: FnOnce(&mut TransactionState) -> U,
    {
        self.database.get_transaction_mut(tx_id).map_or(
            Err(TransactionHandlerError::ExpectedTransactionToExist),
            |txn| {
                if client_id != txn.client_id {
                    return Err(TransactionHandlerError::ExpectedClientIdToMatch);
                }

                Ok(f(txn))
            },
        )
    }

    pub fn get_database(self) -> T {
        self.database
    }
}

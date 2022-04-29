use super::models::transaction::Transaction;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PaymentEngineError {
    ExpectedAmount(Transaction),
    NotEnoughFunds(Transaction),
    ExpectedTransactionToExist(Transaction),
}

impl std::error::Error for PaymentEngineError {}

impl fmt::Display for PaymentEngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PaymentEngineError::ExpectedAmount(t) => write!(
                f,
                "error on transaction #{}: type '{:?}' is expected to have an amount",
                t.tx_id, t.tx_type
            ),
            PaymentEngineError::NotEnoughFunds(t) => write!(
                f,
                "error on transaction #{}: account did not have enough available funds to withdraw an amount of {}",
                t.tx_id, t.get_amt()
            ),
            PaymentEngineError::ExpectedTransactionToExist(t) => write!(
                f,
                "error on transaction of type '{:?}': tx id '{}' does not exist",
                t.tx_type, t.tx_id
            )
        }
    }
}

pub type PaymentEngineResult<T> = Result<T, PaymentEngineError>;

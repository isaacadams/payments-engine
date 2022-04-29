use super::models::transaction::Transaction;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PaymentEngineError {
    TransactionHandler(TransactionHandlerError, Transaction),
}

#[derive(Debug, PartialEq)]
pub enum TransactionHandlerError {
    ExpectedAmount,
    NotEnoughFunds,
    ExpectedTransactionToExist,
    ExpectedClientIdToMatch,
    MustBeInActiveDispute,
}

impl std::error::Error for PaymentEngineError {}

impl fmt::Display for PaymentEngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PaymentEngineError::TransactionHandler(e, t) => {
                match e {
                    TransactionHandlerError::ExpectedAmount => write!(
                        f,
                        "error on transaction #{}: type '{:?}' is expected to have an amount",
                        t.tx_id, t.tx_type
                    ),
                    TransactionHandlerError::NotEnoughFunds => write!(
                        f,
                        "error on transaction #{}: account did not have enough available funds to withdraw an amount of {}",
                        t.tx_id, t.get_amt()
                    ),
                    TransactionHandlerError::ExpectedTransactionToExist => write!(
                        f,
                        "error on transaction of type '{:?}': tx id '{}' does not exist",
                        t.tx_type, t.tx_id
                    ),
                    TransactionHandlerError::ExpectedClientIdToMatch => write!(
                        f,
                        "error on transaction of type '{:?}': specified client id '{}' did not match the client id for tx #{}",
                        t.tx_type, t.client_id, t.tx_id
                    ),
                    TransactionHandlerError::MustBeInActiveDispute => write!(
                        f,
                        "error on transaction of type '{:?}': tx #{} was not in dispute; resolution is not relevant unless transaction is being disputed",
                        t.tx_type, t.tx_id
                    ),
                }
            },
        }
    }
}

pub type PaymentEngineResult<T> = Result<T, PaymentEngineError>;

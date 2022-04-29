use super::models::transaction::{Transaction, TransactionType};
//use super::services::account_state::AccountState;
use super::Database;
use super::{PaymentEngineError, PaymentEngineResult};

pub fn handle_transaction(d: &mut impl Database, x: Transaction) -> PaymentEngineResult<()> {
    match x.tx_type {
        TransactionType::Withdrawal => {
            if let Some(amt) = x.amt {
                let account = d.fetch_client_mut(x.client_id);
                if account.withdraw(amt) {
                    d.add_transaction(x);
                } else {
                    return Err(PaymentEngineError::NotEnoughFunds(x));
                }
            } else {
                return Err(PaymentEngineError::ExpectedAmount(x));
            }
        }
        TransactionType::Deposit => {
            if let Some(amt) = x.amt {
                let account = d.fetch_client_mut(x.client_id);
                account.deposit(amt);
                d.add_transaction(x);
            } else {
                return Err(PaymentEngineError::ExpectedAmount(x));
            }
        }
        TransactionType::Dispute => {
            if let Some(amt) = d.get_transaction_amt(x.tx_id) {
                let account = d.fetch_client_mut(x.client_id);
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

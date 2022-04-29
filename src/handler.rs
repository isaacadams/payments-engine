use super::models::transaction::{Transaction, TransactionType};
//use super::services::account_state::AccountState;
use super::Database;

pub fn handle_transaction(d: &mut impl Database, x: Transaction) {
    match x.tx_type {
        TransactionType::Withdrawal => {
            let account = d.fetch_client_mut(x.client_id);
            account.withdraw(x.get_amt());
            d.add_transaction(x);
        }
        TransactionType::Deposit => {
            let account = d.fetch_client_mut(x.client_id);
            account.deposit(x.get_amt());
            d.add_transaction(x);
        }
        TransactionType::Dispute => {
            println!("dispute found: {:?}", x);
            if let Some(amt) = d.get_transaction_amt(x.tx_id) {
                println!("disputed txn found: {:?}", amt);
                let account = d.fetch_client_mut(x.client_id);
                account.dispute(amt);
                return;
            }

            println!("disputed txn {} does not exist", x.tx_id);
        }
        _ => (),
        /*
        TransactionType::Resolve => {},
        TransactionType::Chargeback => {},
        */
    };
}

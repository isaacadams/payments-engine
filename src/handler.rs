use super::models::transaction::{Transaction, TransactionType};
//use super::services::account_state::AccountState;
use super::Database;

pub fn handle_transaction(d: &mut impl Database, x: Transaction) {
    let account = d.fetch_client(x.client_id);
    match x.tx_type {
        TransactionType::Withdrawal => &account.withdraw(x.amt),
        TransactionType::Deposit => &account.deposit(x.amt), 
        _ => &(),
        /*
        TransactionType::Dispute => {},
        TransactionType::Resolve => {},
        TransactionType::Chargeback => {}, 
        */
    };
}

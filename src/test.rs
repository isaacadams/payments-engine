use crate::{
    database::{get_database, Database},
    handler::handle_transaction,
    models::account::Account,
    models::transaction::TransactionType,
    read,
    services::account_state::AccountState,
    PaymentEngineError, Transaction,
};

#[test]
fn test() {
    let mut database = read("transactions.csv").unwrap();

    let client_1: Account = database.fetch_client_ref(1).into();
    let client_2: Account = database.fetch_client_ref(2).into();

    assert!(client_1.total == 5.287655_f32);
    assert!(client_2.total == 10.0_f32);
    assert!(client_1.held == 1_f32);
}

#[test]
fn test_deposit_puts_funds_in_available() {
    let mut database = get_database();

    handle_transaction(
        &mut database,
        Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(100_f32),
        },
    )
    .unwrap();
    let client_1: Account = database.fetch_client_ref(1).into();

    assert_eq!(client_1.available, 100_f32);
    assert_eq!(client_1.total, 100_f32);
}

#[test]
fn test_withdraw_takes_funds_from_available() {
    let mut database = get_database();

    handle_transaction(
        &mut database,
        Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(100_f32),
        },
    )
    .unwrap();
    handle_transaction(
        &mut database,
        Transaction {
            tx_type: TransactionType::Withdrawal,
            client_id: 1,
            tx_id: 2,
            amt: Some(50.5050_f32),
        },
    )
    .unwrap();
    let client_1: Account = database.fetch_client_ref(1).into();

    assert_eq!(client_1.available, 49.4950_f32);
    assert_eq!(client_1.total, 49.4950_f32);
}

#[test]
fn test_dispute_moves_available_to_held() {
    let mut database = get_database();

    handle_transaction(
        &mut database,
        Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(50_f32),
        },
    )
    .unwrap();

    handle_transaction(
        &mut database,
        Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 2,
            amt: Some(50_f32),
        },
    )
    .unwrap();

    handle_transaction(
        &mut database,
        Transaction {
            tx_type: TransactionType::Dispute,
            client_id: 1,
            tx_id: 2,
            amt: None,
        },
    )
    .unwrap();

    let client_1: Account = database.fetch_client_ref(1).into();

    assert_eq!(client_1.available, 50_f32);
    assert_eq!(client_1.held, 50_f32);
    assert_eq!(client_1.total, 100_f32);
}

#[test]
fn test_deposit_throws_expected_amt_err() {
    let mut database = get_database();
    let tx = Transaction {
        tx_type: TransactionType::Deposit,
        client_id: 1,
        tx_id: 1,
        amt: None,
    };

    assert_eq!(
        PaymentEngineError::ExpectedAmount(tx.clone()),
        handle_transaction(&mut database, tx.clone()).unwrap_err()
    );
}

#[test]
fn test_withdraw_throws_expected_amt_err() {
    let mut database = get_database();
    let tx = Transaction {
        tx_type: TransactionType::Withdrawal,
        client_id: 1,
        tx_id: 1,
        amt: None,
    };

    assert_eq!(
        PaymentEngineError::ExpectedAmount(tx.clone()),
        handle_transaction(&mut database, tx.clone()).unwrap_err()
    );
}

#[test]
fn test_withdraw_throws_not_enough_funds_err() {
    let mut database = get_database();

    handle_transaction(
        &mut database,
        Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(50_f32),
        },
    )
    .unwrap();

    handle_transaction(
        &mut database,
        Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 2,
            amt: Some(50_f32),
        },
    )
    .unwrap();

    handle_transaction(
        &mut database,
        Transaction {
            tx_type: TransactionType::Dispute,
            client_id: 1,
            tx_id: 1,
            amt: None,
        },
    )
    .unwrap();

    let client_1: Account = database.fetch_client_ref(1).into();

    assert_eq!(client_1.available, 50_f32);
    assert_eq!(client_1.held, 50_f32);
    assert_eq!(client_1.total, 100_f32);

    let tx = Transaction {
        tx_type: TransactionType::Withdrawal,
        client_id: 1,
        tx_id: 3,
        amt: Some(75_f32),
    };

    assert_eq!(
        PaymentEngineError::NotEnoughFunds(tx.clone()),
        handle_transaction(&mut database, tx.clone()).unwrap_err()
    );
}

#[test]
fn test_dispute_throws_expected_transaction_to_exist() {
    let mut database = get_database();
    let tx = Transaction {
        tx_type: TransactionType::Dispute,
        client_id: 1,
        tx_id: 1,
        amt: None,
    };

    assert_eq!(
        PaymentEngineError::ExpectedTransactionToExist(tx.clone()),
        handle_transaction(&mut database, tx.clone()).unwrap_err()
    );
}
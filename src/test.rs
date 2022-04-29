use crate::{
    database::{get_database, Database, InMemoryDatabase},
    handler::TransactionHandler,
    models::account::Account,
    models::transaction::TransactionType,
    read,
    PaymentEngineError,
    Transaction,
    TransactionHandlerError,
};

#[test]
fn test() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();
    read(&mut transaction_handler, "transactions.csv").unwrap();

    let mut database = transaction_handler.get_database();
    let client_1: Account = database.fetch_client_ref(1).into();

    assert!(client_1.available == 7_f32);
    assert!(client_1.held == 0_f32);
    assert!(client_1.total == 7_f32);
    assert!(client_1.locked == true);

    let client_2: Account = database.fetch_client_ref(2).into();

    assert!(client_2.available == 4.0_f32);
    assert!(client_2.held == 6.0_f32);
    assert!(client_2.total == 10.0_f32);
    assert!(client_2.locked == false);

    let client_3: Account = database.fetch_client_ref(3).into();

    assert!(client_3.available == 10.0_f32);
    assert!(client_3.held == 0.0_f32);
    assert!(client_3.total == 10.0_f32);
    assert!(client_3.locked == false);
}

#[test]
fn test_deposit_puts_funds_in_available() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(100_f32),
        })
        .unwrap();

    let mut database = transaction_handler.get_database();
    let client_1: Account = database.fetch_client_ref(1).into();

    assert_eq!(client_1.available, 100_f32);
    assert_eq!(client_1.total, 100_f32);
}

#[test]
fn test_withdraw_takes_funds_from_available() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(100_f32),
        })
        .unwrap();
    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Withdrawal,
            client_id: 1,
            tx_id: 2,
            amt: Some(50.5050_f32),
        })
        .unwrap();

    let mut database = transaction_handler.get_database();
    let client_1: Account = database.fetch_client_ref(1).into();

    assert_eq!(client_1.available, 49.4950_f32);
    assert_eq!(client_1.total, 49.4950_f32);
}

#[test]
fn test_dispute_moves_available_to_held() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(50_f32),
        })
        .unwrap();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 2,
            amt: Some(50_f32),
        })
        .unwrap();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Dispute,
            client_id: 1,
            tx_id: 2,
            amt: None,
        })
        .unwrap();

    let mut database = transaction_handler.get_database();
    let client_1: Account = database.fetch_client_ref(1).into();

    assert_eq!(client_1.available, 50_f32);
    assert_eq!(client_1.held, 50_f32);
    assert_eq!(client_1.total, 100_f32);
}

#[test]
fn test_resolve_moves_held_back_to_available() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(50_f32),
        })
        .unwrap();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 2,
            amt: Some(50_f32),
        })
        .unwrap();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Dispute,
            client_id: 1,
            tx_id: 2,
            amt: None,
        })
        .unwrap();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Resolve,
            client_id: 1,
            tx_id: 2,
            amt: None,
        })
        .unwrap();

    let mut database = transaction_handler.get_database();
    let client_1: Account = database.fetch_client_ref(1).into();

    assert_eq!(client_1.available, 100_f32);
    assert_eq!(client_1.held, 0_f32);
    assert_eq!(client_1.total, 100_f32);
}

#[test]
fn test_chargeback_moves_held_back_to_available_and_locks_account() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(50_f32),
        })
        .unwrap();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 2,
            amt: Some(50_f32),
        })
        .unwrap();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Dispute,
            client_id: 1,
            tx_id: 2,
            amt: None,
        })
        .unwrap();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Chargeback,
            client_id: 1,
            tx_id: 2,
            amt: None,
        })
        .unwrap();

    let mut database = transaction_handler.get_database();
    let client_1: Account = database.fetch_client_ref(1).into();

    assert_eq!(client_1.available, 50_f32);
    assert_eq!(client_1.held, 0_f32);
    assert_eq!(client_1.total, 50_f32);
    assert_eq!(client_1.locked, true);
}

#[test]
fn test_deposit_throws_expected_amt_err() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();
    let tx = Transaction {
        tx_type: TransactionType::Deposit,
        client_id: 1,
        tx_id: 1,
        amt: None,
    };

    assert_eq!(
        PaymentEngineError::TransactionHandler(TransactionHandlerError::ExpectedAmount, tx.clone()),
        transaction_handler.handle(tx.clone()).unwrap_err()
    );
}

#[test]
fn test_withdraw_throws_expected_amt_err() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();
    let tx = Transaction {
        tx_type: TransactionType::Withdrawal,
        client_id: 1,
        tx_id: 1,
        amt: None,
    };

    assert_eq!(
        PaymentEngineError::TransactionHandler(TransactionHandlerError::ExpectedAmount, tx.clone()),
        transaction_handler.handle(tx.clone()).unwrap_err()
    );
}

#[test]
fn test_withdraw_throws_not_enough_funds_err() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(50_f32),
        })
        .unwrap();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 2,
            amt: Some(50_f32),
        })
        .unwrap();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Dispute,
            client_id: 1,
            tx_id: 1,
            amt: None,
        })
        .unwrap();

    let tx = Transaction {
        tx_type: TransactionType::Withdrawal,
        client_id: 1,
        tx_id: 3,
        amt: Some(75_f32),
    };

    assert_eq!(
        PaymentEngineError::TransactionHandler(TransactionHandlerError::NotEnoughFunds, tx.clone()),
        transaction_handler.handle(tx.clone()).unwrap_err()
    );

    let mut database = transaction_handler.get_database();
    let client_1: Account = database.fetch_client_ref(1).into();

    assert_eq!(client_1.available, 50_f32);
    assert_eq!(client_1.held, 50_f32);
    assert_eq!(client_1.total, 100_f32);
}

#[test]
fn test_dispute_throws_expected_transaction_to_exist() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();

    let tx = Transaction {
        tx_type: TransactionType::Dispute,
        client_id: 1,
        tx_id: 1,
        amt: None,
    };

    assert_eq!(
        PaymentEngineError::TransactionHandler(
            TransactionHandlerError::ExpectedTransactionToExist,
            tx.clone()
        ),
        transaction_handler.handle(tx.clone()).unwrap_err()
    );
}

#[test]
fn test_resolve_throws_expected_transaction_to_exist() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();

    let tx = Transaction {
        tx_type: TransactionType::Resolve,
        client_id: 1,
        tx_id: 1,
        amt: None,
    };

    assert_eq!(
        PaymentEngineError::TransactionHandler(
            TransactionHandlerError::ExpectedTransactionToExist,
            tx.clone()
        ),
        transaction_handler.handle(tx.clone()).unwrap_err()
    );
}

#[test]
fn test_dispute_throws_expected_client_id_to_match() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(50_f32),
        })
        .unwrap();

    let tx = Transaction {
        tx_type: TransactionType::Dispute,
        client_id: 2,
        tx_id: 1,
        amt: None,
    };

    assert_eq!(
        PaymentEngineError::TransactionHandler(
            TransactionHandlerError::ExpectedClientIdToMatch,
            tx.clone()
        ),
        transaction_handler.handle(tx.clone()).unwrap_err()
    );
}

#[test]
fn test_resolve_throws_expected_client_id_to_match() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(50_f32),
        })
        .unwrap();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Dispute,
            client_id: 1,
            tx_id: 1,
            amt: None,
        })
        .unwrap();

    let tx = Transaction {
        tx_type: TransactionType::Resolve,
        client_id: 2,
        tx_id: 1,
        amt: None,
    };

    assert_eq!(
        PaymentEngineError::TransactionHandler(
            TransactionHandlerError::ExpectedClientIdToMatch,
            tx.clone()
        ),
        transaction_handler.handle(tx.clone()).unwrap_err()
    );
}

#[test]
fn test_resolve_throws_must_be_in_active_dispute() {
    let mut transaction_handler: TransactionHandler<InMemoryDatabase> = get_database().into();

    transaction_handler
        .handle(Transaction {
            tx_type: TransactionType::Deposit,
            client_id: 1,
            tx_id: 1,
            amt: Some(50_f32),
        })
        .unwrap();

    let tx = Transaction {
        tx_type: TransactionType::Resolve,
        client_id: 1,
        tx_id: 1,
        amt: None,
    };

    assert_eq!(
        PaymentEngineError::TransactionHandler(
            TransactionHandlerError::MustBeInActiveDispute,
            tx.clone()
        ),
        transaction_handler.handle(tx.clone()).unwrap_err()
    );
}

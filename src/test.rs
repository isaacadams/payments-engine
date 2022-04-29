use super::read;
use crate::{database::Database, models::account::Account, services::account_state::AccountState};

#[test]
fn test() {
    let mut database = read("transactions.csv").unwrap();

    let client_1: Account = database.fetch_client_ref(1).into();
    let client_2: Account = database.fetch_client_ref(2).into();

    assert!(client_1.total == 5.287655_f32);
    assert!(client_2.total == 10.0_f32);
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    #[serde(alias = "withdraw")]
    Withdrawal,
    Deposit,
    Dispute,
    Resolve,
    Chargeback
}


#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    tx_type: TransactionType,
    #[serde(rename = "client")]
    client_id: u16,
    #[serde(rename = "tx")]
    tx_id: u16,
    amt: u32,
}
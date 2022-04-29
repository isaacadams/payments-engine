use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    #[serde(alias = "withdraw")]
    Withdrawal,
    Deposit,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: TransactionType,
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub tx_id: u32,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub amt: Option<f32>,
}

impl Transaction {
    pub fn get_amt(&self) -> f32 {
        match &self.amt {
            Some(x) => *x,
            None => 0_f32,
        }
    }
}

use std::{env, error::Error};

#[cfg(test)]
mod test;

mod database;
mod error;
mod handler;
mod models;
mod services;

use database::{Database, InMemoryDatabase};
use error::{PaymentEngineError, PaymentEngineResult};
use handler::TransactionHandler;
use models::transaction::Transaction;

fn main() {
    let args: Vec<String> = env::args().collect();
    let transactions_path = &args[1];

    let mut transaction_handler: TransactionHandler<InMemoryDatabase> =
        database::get_database().into();

    match read(&mut transaction_handler, &transactions_path) {
        Err(e) => {
            println!(
                "failed to read csv file: {}\n   -> error: {}",
                transactions_path, e
            );
        }
        Ok(_) => {
            println!("{:?}", transaction_handler.get_database().get_accounts());
        }
    }
}

pub fn read<T: Database>(
    transaction_handler: &mut TransactionHandler<T>,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .has_headers(true)
        .from_path(path)?;

    for result in rdr.deserialize() {
        let record: Transaction = result?;
        println!("{:?}", record);
        if let Err(e) = &transaction_handler.handle(record) {
            println!("{}", e);
        }
    }

    Ok(())
}

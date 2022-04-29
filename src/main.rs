use std::{env, error::Error};

#[cfg(test)]
mod test;

mod database;
mod error;
mod handler;
mod models;
mod services;

use database::{Database, InMemoryDatabase};
use error::{PaymentEngineError, PaymentEngineResult, TransactionHandlerError};
use handler::TransactionHandler;
use models::{account::Account, transaction::Transaction};

fn main() {
    let args: Vec<String> = env::args().collect();
    let transactions_path = &args[1];

    let mut transaction_handler: TransactionHandler<InMemoryDatabase> =
        database::get_database().into();

    if let Some(e) = read(&mut transaction_handler, transactions_path).err() {
        println!(
            "failed to read csv file: {}\n   -> error: {}",
            transactions_path, e
        );
        std::process::exit(1);
    }

    if let Some(e) = write_csv_output(transaction_handler.get_database().get_accounts()).err() {
        println!("failed to write csv output\n   -> error: {}", e);
        std::process::exit(1);
    }
}

fn write_csv_output(accounts: Vec<Account>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(std::io::stdout());

    for a in accounts {
        writer.serialize(a)?;
    }

    writer.flush()?;

    Ok(())
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
        //println!("{:?}", record);
        // ignore handler errors
        if transaction_handler.handle(record).is_err() {
            //println!("{}", e);
        }
    }

    Ok(())
}

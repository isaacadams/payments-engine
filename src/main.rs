use std::{env, error::Error};

mod database;
mod handler;
mod models;
mod services;

use database::Database;
use models::transaction::Transaction;

fn main() {
    let args: Vec<String> = env::args().collect();
    let transactions_path = &args[1];

    match read(&transactions_path) {
        Err(e) => {
            println!(
                "failed to read csv file: {}\n   -> error: {}",
                transactions_path, e
            );
        }
        Ok(d) => {
            println!("{:?}", d.get_accounts());
        }
    }
}

fn read(path: &str) -> Result<impl Database, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .has_headers(true)
        .from_path(path)?;

    let mut d = database::get_database();

    for result in rdr.deserialize() {
        let record: Transaction = result?;
        println!("{:?}", record);
        handler::handle_transaction(&mut d, record);
    }

    Ok(d)
}

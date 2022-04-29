use std::{env, error::Error};

mod models;
mod services;
mod database;
mod handler;

use models::transaction::Transaction;
use database::Database;

fn main() {
    let args: Vec<String> = env::args().collect();
    let transactions_path = &args[1];

    if let Err(err) = read(&transactions_path) {
        println!(
            "failed to read csv file: {}\n   -> error: {}",
            transactions_path, err
        );
    }
}

fn read(path: &str) -> Result<(), Box<dyn Error>> {
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

    d.print();

    Ok(())
}

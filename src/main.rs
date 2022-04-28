use std::{env, error::Error};

mod models;

use models::transaction::{Transaction};

fn main() {
    let args: Vec<String> = env::args().collect();
    let transactions_path = &args[1];

    if let Err(err) = read(&transactions_path) {
        println!("failed to read csv file: {}\n   -> error: {}", transactions_path, err);
    }
}


fn read(path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .has_headers(true)
        .from_path(path)?;

    for result in rdr.deserialize() {
        let record: Transaction = result?;
        println!("{:?}", record);
    }

    Ok(())
}
use std::{env, error::Error};

mod transaction;

use transaction::{Transaction};

fn main() {
    let args: Vec<String> = env::args().collect();
    let transactions_path = &args[1];
    println!("{:?}", &transactions_path);

    if let Err(err) = read(&transactions_path) {
        println!("error running example: {}", err);
    }
}


fn read(path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    for result in rdr.deserialize() {
        let record: Transaction = result?;
        println!("{:?}", record);
    }

    Ok(())
}
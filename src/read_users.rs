use std::error::Error;
use std::io;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct UserRecord {
    first_name: String,
    last_name: String,
    username: String,
    balance: f64,
    password: String,
}

pub fn read_users_from_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let user: UserRecord = result?;
        println!("{:?}", user);
    }
    Ok(())
}

/*
fn exmaple() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

// then in main:

if let Err(err) = example() {
    println!("error running example: {}", err);
    process::exit(1);
}
*/

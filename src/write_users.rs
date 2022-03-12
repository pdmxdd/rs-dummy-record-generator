use std::error::Error;
use std::io;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
struct UserRecord {
    first_name: String,
    last_name: String,
    username: String,
    balance: f64,
    password: String,
}

pub fn write_users_to_csv() -> Result<(), Box<dyn Error>> {
    let mut wrt = csv::Writer::from_writer(io::stdout());

    wrt.serialize(UserRecord {
        first_name: "Tom".to_string(),
        last_name: "Haverford".to_string(),
        username: "thaverford789".to_string(),
        balance: 500.00,
        password: "password".to_string(),
    })?;

    wrt.flush()?;
    Ok(())
}
use fake::locales::EN;
use fake::Fake;
use fake::faker::name::raw::{FirstName, LastName};
use fake::faker::internet::raw::{Username};
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn random_password() -> String {
    if let Ok(mut lines) = read_lines("10-million-password-list-top-1000000.txt") {
        let password = lines.nth(rand::thread_rng().gen_range(0..999997))
            .expect("File does not exist")
            .expect("File empty")
            .to_string();
        let password = &password;
        return password.to_string();
        // return lines.nth(rand::thread_rng().gen_range(0..999997)).unwrap().unwrap().to_string();
    }
    return "".to_string();
}

pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub balance: f64,
    pub password: String,
}

impl User {
    pub fn new() -> Self {
        Self {
            first_name: FirstName(EN).fake(),
            last_name: LastName(EN).fake(),
            username: Username(EN).fake(),
            balance: rand::thread_rng().gen_range(1.00..7500.00),
            password: random_password(),
        }
    }
}

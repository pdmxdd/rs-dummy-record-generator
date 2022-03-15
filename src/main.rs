use std::process;

mod generate;
use generate::User;

mod read_users;

use read_users::read_users_from_csv;

mod write_users;

use write_users::write_users_to_csv;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rand::Rng;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn random_password() -> String {
    if let Ok(mut lines) = read_lines("10-million-password-list-top-1000000.txt") {
        let password = lines.nth(rand::thread_rng().gen_range(0..999997)).unwrap().unwrap().to_string();
        let password = &password[0..password.len()];
        return password.to_string();
        // return lines.nth(rand::thread_rng().gen_range(0..999997)).unwrap().unwrap().to_string();
    }
    return "".to_string();
}

fn main() { 
    
    println!("{}", random_password());

    // if let Ok(mut lines) = read_lines("10-million-password-list-top-1000000.txt") {
    //     // number of records 999997
    //     println!("{:?}", &lines.nth(rand::thread_rng().gen_range(0..999997)).unwrap().unwrap().to_string());
    // }

    // let user: User = User::new();
    // println!("first_name: {}\nlast_name: {}\nusername: {}\nbalance: {}", user.first_name, user.last_name, user.username, user.balance);
    
    // if let Err(err) = read_users_from_csv() { 
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }

    // if let Err(err) = write_users_to_csv() {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }
}

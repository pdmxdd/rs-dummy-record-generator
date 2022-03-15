use std::process;
use std::process::Command;

mod generate;
use generate::User;

mod read_users;

use read_users::read_users_from_csv;

mod write_users;

use write_users::write_users_to_csv;

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process");

    let hello = output.stdout;
    println!("{:?}", hello);
    
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

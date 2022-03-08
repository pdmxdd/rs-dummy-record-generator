use fake::locales::EN;
use fake::Fake;
use fake::faker::name::raw::*;
use fake::faker::internet::raw::*;
use rand::Rng;

pub struct User {
    first_name: String,
    last_name: String,
    username: String,
    balance: f64,
}

impl User {
    pub fn new() -> Self {
        Self {
            first_name: FirstName(EN).fake(),
            last_name: LastName(EN).fake(),
            username: Username(EN).fake(),
            balance: rand::thread_rng().gen_range(1.00..25000.00),
        }
    }
}

/*
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "first_name: {}\nlast_name: {}\nusername: {}\nbalance: {}", self.first_name, self.last_name, self.username, self.balance);
    }
}
*/


fn main() {
    let user: User = User::new();
    // println!(user);
    println!("first_name: {}\nlast_name: {}\nusername: {}\nbalance: {}", user.first_name, user.last_name, user.username, user.balance);
}

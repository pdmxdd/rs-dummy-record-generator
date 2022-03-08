use fake::locales::EN;
use fake::Fake;
use fake::faker::name::raw::*;
use fake::faker::internet::raw::*;

pub struct User {
    first_name: String,
    last_name: String,
    username: String,
}

impl User {
    fn new() -> Self {
        User {
            first_name: FirstName(EN).fake(),
            last_name: LastName(EN).fake(),
            username: Username(EN).fake(),
        }
    }
}


fn main() {
    let user: User = User::new();
    println!("first_name: {}\nlast_name: {}\nusername: {}", user.first_name, user.last_name, user.username);
}

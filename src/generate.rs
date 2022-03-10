use fake::locales::EN;
use fake::Fake;
use fake::faker::name::raw::{FirstName, LastName};
use fake::faker::internet::raw::{Username};
use rand::Rng;

pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub balance: f64,
}

impl User {
    pub fn new() -> Self {
        Self {
            first_name: FirstName(EN).fake(),
            last_name: LastName(EN).fake(),
            username: Username(EN).fake(),
            balance: rand::thread_rng().gen_range(1.00..7500.00),
        }
    }
}

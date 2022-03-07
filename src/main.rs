use fake::locales::EN;
use fake::Fake;

fn main() {
    use fake::faker::name::raw::*;
    let first_name: String = FirstName(EN).fake();
    let last_name: String = LastName(EN).fake();
    println!("first: {}\nlast: {}", first_name, last_name);
}

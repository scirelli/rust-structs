#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = User{
        active: true,
        username: String::from("Hello"),
        email: String::from("bob@bobsicle@gmail.com"),
        sign_in_count: 0
    };
    let user2 = User{
        ..user
    };
    println!("{user:?}\n{user2:?}");
}

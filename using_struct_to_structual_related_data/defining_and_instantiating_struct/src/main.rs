// declaring a struct
#[derive(Debug)]
struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        user_name: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    let user2 = build_user(
        String::from("someusername123"),
        String::from("game@changer.com"),
    );
    println!("print the user out {:#?}", user1);
    println!("print the user out {:#?}", user2);
}

fn build_user(user_name: String, email: String) -> User {
    User {
        active: true,
        user_name,
        email,
        sign_in_count: 1,
    }
}

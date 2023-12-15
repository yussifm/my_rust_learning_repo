// a struct of user
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_counts: u64,
}

pub fn structures_in_rust() {

    let user_1: User = User {
        active: true,
        username: String::from("codeStudios"),
        email: String::from("user@gmail.com"),
        sign_in_counts: 1,
    };

    println!("User Name: {}", user_1.username);
    println!("Is User active: {}", user_1.active);
    println!("User Email: {}", user_1.email);
    println!("Number of signs in: {}", user_1.sign_in_counts);
}

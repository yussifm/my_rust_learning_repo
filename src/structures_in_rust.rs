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

    print!("{}", user_1.username);
    print!("{}", user_1.active);
    print!("{}", user_1.email);
    print!("{}", user_1.sign_in_counts);
}

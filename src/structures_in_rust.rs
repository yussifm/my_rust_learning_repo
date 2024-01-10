// a struct of user
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_counts: u64,
}
struct  AlwaysEqual;

// Tuple struct
struct Color(i32, i32, i32);
struct Points(i32, i32, i32);

pub fn structures_in_rust() {
    let color_1 = Color(0, 1, 2);
    let point_1 = Points(0, 0, 0);

    let user_1: User = User {
        active: true,
        username: String::from("codeStudios"),
        email: String::from("user@gmail.com"),
        sign_in_counts: 1,
    };

    // Unit-Like Structs Without Any Fields
    //A structs that doesn't have any fields.
    //Unit-Like structs can be useful when you
    //need to implement a trait on some type but
    // don't have any data that you want to store in the type itself.
    let subject = AlwaysEqual;



    println!("======== Normal Struct ==========",);
    println!("User Name: {}", user_1.username);
    println!("Is User active: {}", user_1.active);
    println!("User Email: {}", user_1.email);
    println!("Number of signs in: {}", user_1.sign_in_counts);

    println!("======== Tuples Struct ==========",);
    println!("tuple struct with color example: {}", color_1.1);
    println!("tuple struct with points example: {}", point_1.0);
}

pub fn ownership_rs_tuto() {
    // Variable Scope

    let mut s = String::from("Hello");

    // {  // s is not declare here and not available
    //     let s = "Coded"; // s is declare here and available
    //
    // } // this scope is now over, and s is no longer valid

    //Memory and Allocations
    s.push_str(", World");
    let s1 = s;
    println!("{}", s1);

    //variable and Data Interacting clone
    let mut s2 = String::from("Hello");
    let s3 = s2.clone();

    println!("s2 = {}, s3 = {} \n", s2, s3);

    // Stack only data: copy

    // we don't need clone for integers because their size is known.
    // They are Store on the Stack
    let x = 50_000;
    let y = x;
    println!("Y value: {}", y);

    // Ownership and Functions

    let s4 = String::from("Hello"); // s4 comes into scope

    takes_ownership(s4); // s4's value moves into the function and so is no longer valid here

    let x1 = 5; // x1 comes into scope

    makes_copy(x1); // x1 would move into the function,
                    // but i32 is copy, so it's okay to still use x afterward

    // Return Values and Scope
    let s5 = gives_ownership(); // gives_ownership moves its return value into s5

    let s6 = String::from("hello"); // s6 comes into scope

    let s7 = takes_and_gives_back(s6); // s6 is moved into
                                       // takes_and_gives_back, which also moves its return value into s7
}

fn takes_ownership(some_string_string: String) {
    println!("Takes ownership value: {}", some_string_string);
}

fn makes_copy(some_integer: i32) {
    println!("Makes a copy value: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

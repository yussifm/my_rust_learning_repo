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
}

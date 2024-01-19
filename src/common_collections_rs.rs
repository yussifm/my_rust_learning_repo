use std::collections::HashMap;

pub fn common_collections_in_rs() {
    // Vectors
    vectors_collections();

    // String collections
    strings_collections_in_rs();

    // Hash Maps
    hashMaps_collections_in_rs();
}

fn vectors_collections() {
    // Vectors

    // Creating new vectors

    let v: Vec<i32> = Vec::new();
    let v1 = vec!["1", "2", "3", "4", "5"];

    println!("The length of v vector is: {}", &v.len());

    //  updating a vector
    let mut v2: Vec<i32> = Vec::new();

    v2.push(1);
    v2.push(2);
    v2.push(3);

    // reading elements of a vector
    let third_value = &v1[2];
    println!("The third value is: {third_value}");

    let opt_value: Option<&i32> = v2.get(1);
    match opt_value {
        Some(value) => println!("The value is : {value}"),
        None => println!("Value is out of range"),
    }

    // Iterating over values in a vector
    println!(
        "====================== Iterating over values in a vector ============================="
    );
    for i in &v2 {
        println!("{i}");
    }

    // Iterating over values and changing their values
    println!("====================== Iterating over values and changing their values =============================");
    for i in &mut v2 {
        *i += 5;
        println!("{i}");
    }

    //  Using an Enum to Store Multiple Types
    enum SpreadSheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }
    let v3 = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(15.3),
        SpreadSheetCell::Text(String::from("Hello")),
    ];

    // Dropping a Vector Drops its Elements
    {
        let v = vec![1, 2, 3, 4, 5, 6];
        println!("========= {}", &v[0]); // do stuff with v
    } // <- v goes out of scope and is freed here
}

fn strings_collections_in_rs() {
    // Strings collections

    // Creating a new string
    let mut s = String::new();
    let data = "Hello from string collection";
    let s1 = data.to_string();

    // also works on literals directly
    let s2 = "Literal string ".to_string();
    let s3 = String::from("initial contents");

    // Strings are UTF-8 encoded,  so we can include any properly encoded data in them
    let hello = String::from("عليكم السالم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("וםֹל שָׁ ");
    println!("{}", hello);
    let hello = String::from("नमस्ते ");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    // Updating a String
    // Appending to a String with push_str and push
    s.push_str("hello");

    // Concatenating a String with the '+' Operator or format! Macro
    let s4 = String::from("Hello, ");
    let s5 = String::from("World!");

    let s6 = s4 + &s5; // s4 have been moved here and can't longer be used
    println!("{} {} {} {} {} ", s1, s2, s3, s, s6);

    // Format! macro
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");
    let s10 = format!("{s7}-{s8}-{s9}");

    println!("{} ", s10);

    // Indexing into Strings

    // Rust strings don't support indexing.
    //    let h = s10[0];

    // Bytes and Scalar Values and Grapheme Clusters

    // Another point about UTF-8 is that there are actually
    // three relevant ways to look at strings from
    //Rust’s perspective: as bytes, scalar values, and grapheme clusters
    // (the closest thing to what we would call letters).

    // String slice

    let hello = "Здравствуйте";
    let s11 = &hello[0..4];

    // Methods for Iterating Over Strings
    for c in "Hello World".chars() {
        println!("{c}");
    }

    for b in "السالم".bytes() {
        println!("{b}");
    }
}

fn hashMaps_collections_in_rs() {
    //  Storing Keys with Associated Values in Hash Maps

    // Creating a new Hash Map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

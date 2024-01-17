pub fn common_collections_in_rs() {
  // Vectors
  vectors_collections();

  // String collections
   strings_collections_in_rs();


}

fn vectors_collections (){
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
        println!("========= {}", &v[0]);    // do stuff with v

     
    } // <- v goes out of scope and is freed here

}


fn strings_collections_in_rs(){

   // Strings collections  
}
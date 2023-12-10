pub fn variables_in_rust() {
    // Mutable variables: {v2}
    let v2 = 4;

    // UnMutable variables: {v3}
    let v3 = 6;

    //nMutable and Const variables: {VCONST}
    const VCONST: u32 = 60 * 60 * 4;

    //tuples
    let tup: (i32, i64, f64, u8) = (2, 100, 30.67, 1);

    // Array
    let arr_type_one = [0, 1, 2, 3, 4, 5, 6];
    let arr_type_two: [i64; 5] = [30, 10, 20, 60, 40];

    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    println!("Mutable variables: {v2}");
    println!("UnMutable variables: {v3}");
    println!("UnMutable and Const variables: {VCONST}");

    // using spread on tuples
    let (a, b, c, d) = tup;

    // getting single tuple values
    let y = tup.0;
    let z = tup.3;
    println!("tuples {a}");
    println!("tuples {b}");
    println!("tuples {c}");
    println!("tuples {d}");
    println!("======= Single tuple values ============");
    println!("tuples {y}");
    println!("tuples {z}");

    println!("=========== Array values ============");
    let fist = arr_type_one[0];
    let second = arr_type_two[0];
    println!("Array One: {fist} ");
    println!("Array One: {second}");
    let add_num = add(33.0, 40.0);

    println!("===========  Functions ============");

    println!("Add function : {add_num}")
}

// Functions in Rust
fn add(x: f64, y: f64) -> f64 {
    // Valid code return
    //    let result = x + y;
    //     return result;

    // valid code retune
    x + y
}

pub fn references_main_func() {
    let s = String::from("Coded");

    let len = calculate_length(&s); // &s reference of s instead moving it
                                    // The ampersands represent references, and it allow you to
                                    // refer to some value without taking ownership of it
    println!("The length of {} String is, {}", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

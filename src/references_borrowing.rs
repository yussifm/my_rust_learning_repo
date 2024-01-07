pub fn references_main_func() {
    let s = String::from("Coded");

    let len = calculate_length(&s);
    println!("The length of {} String is, {}", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

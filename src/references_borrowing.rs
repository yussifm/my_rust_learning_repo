// & reference
// * dereferences

pub fn references_main_func() {
    let s = String::from("Coded");

    let len = calculate_length(&s); // &s reference of s instead moving it
                                    // The ampersands represent references, and it allow you to
                                    // refer to some value without taking ownership of it
                                    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
                                    // Because it does not own it, the value it points to will not be dropped when the reference stops
                                    // being used.
    println!("The length of {} String is, {}", s, len);

    // Mutables References
    let mut s1 = String::from("Hello");
    let mStr = change_reference(&mut s1);
    println!("mutate String, {}", mStr);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_reference(s: &mut String) -> &mut String {
    s.push_str(", World");
    s
}

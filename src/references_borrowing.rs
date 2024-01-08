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

    // Mutable references have one big restriction: if you have a mutable reference to a value, you
    // can have no other references to that value.

    let mut s1 = String::from("Hello");
    let mStr = change_reference(&mut s1);
    println!("mutate String, {}", mStr);

    // DANGLING References
    // In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that
    //references a location in memory that may have been given to someone else—by freeing some
    //memory while preserving a pointer to that memory. In Rust, by contrast, the compiler
    //guarantees that references will never be dangling references: if you have a reference to some
    //data, the compiler will ensure that the data will not go out of scope before the reference to the
    //data does.


    // The Slice Type

}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_reference(s: &mut String) -> &mut String {
    s.push_str(", World");
    s
}

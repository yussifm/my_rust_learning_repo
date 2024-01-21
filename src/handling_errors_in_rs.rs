// # Two major categories of errors in Rust
//  - recoverable error
// - unrecoverable error

use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

pub fn handling_errors_in_rs() {
    // # Unrecoverable errors with panic!
    // -  Unwinding the Stack or Aborting in Response to a Panic

    //  If in your project you need to make the resulting binary as small as possible, you
    // can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the
    // appropriate [profile] sections in your Cargo.toml fi le. For example, if you want to abort
    // on panic in release mode, add this:
    // [profile.release]
    //panic = 'abort'

    // panic!("Crash and burn");

    // - Using a panic backtrace

    // # Recoverable Errors with Result

    // using file open

    let op_file = File::open("hello.txt");
    // The return type of File::open is a Result<T, E> . The generic parameter T has been filled in
    //by the implementation of File::open with the type of the success value, std::fs::File ,
    //which is a file handle. The type of E used in the error value is std::io::Error . This return
    //type means the call to File::open might succeed and return a file handle that we can read
    //from or write to. The function call also might fail: for example, the file might not exist, or we
    //might not have permission to access the file.

    // need to use the match keyword to catch the file error

    let file_result = match op_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    //   -  Alternatives to Using match with Result<T, E>

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}


// Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator

fn read_username_from_file_with_propa() -> Result<String, io::Error> {
let mut username_file = File::open("hello.txt")?;
let mut username = String::new();
username_file.read_to_string(&mut username)?;
Ok(username)
}


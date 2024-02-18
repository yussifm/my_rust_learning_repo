use ::std::cmp::Ordering;
use rand::Rng;
use std::io;
mod area_of_circle;
mod closures_in_rs;
mod common_collections_rs;
mod concurrency_in_rs;
mod enums_in_rust;
mod generics_traaits_in_rs;
mod handling_errors_in_rs;
mod oop_in_rs;
mod ownership_in_rust;
mod references_borrowing;
mod smart_pointers_in_rs;
mod structures_in_rust;
mod traits_in_rs;
mod varials_in_rs;
mod unsafe_rs;

use closures_in_rs::closures_in_rs;
use handling_errors_in_rs::handling_errors_in_rs;
use ownership_in_rust::ownership_rs_tuto;
use references_borrowing::references_main_func;
use smart_pointers_in_rs::smart_pointers;
use structures_in_rust::structures_in_rust;
use traits_in_rs::traits_in_rs;
use varials_in_rs::variables_in_rust;

use area_of_circle::area;
use common_collections_rs::common_collections_in_rs;

use crate::area_of_circle::{area_with_struct, area_with_tuple, Rectangle, RectangleM};
use crate::concurrency_in_rs::{messages_passing_trn, multiple_messages_tr};
use concurrency_in_rs::all_concurrency_exam;
use generics_traaits_in_rs::generics_and_traits_in_rs;
use oop_in_rs::object_orient_program_rs;

fn main() {
    println!("Guess the number!");

    let mut secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        println!("The secret number is: {secret_number}\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You Win🥇🥇");
                println!("Enter 'C' to continue or 'E' to exit");
                let mut opt = String::new();

                io::stdin()
                    .read_line(&mut opt)
                    .expect("Error reading input");

                if opt.to_lowercase().trim() == "c" {
                    secret_number = rand::thread_rng().gen_range(1..=100);
                    continue;
                } else {
                    break;
                }
            }
            Ordering::Greater => {
                println!("Too Big");
            }
        }
    }

    println!("============ Variables In Rust ==============");
    println!("===================================================");
    variables_in_rust();
    println!("===================================================");
    println!("===================================================");
    println!("============ OwnerShip In Rust ==============");
    ownership_rs_tuto();
    println!("===================================================");
    println!("===================================================");
    println!("============ References and Borrowing In Rust ==============");
    references_main_func();
    println!("===================================================");
    println!("===================================================");
    println!("============ Struct / Structures In Rust ==============");
    structures_in_rust();
    println!("===================================================");
    println!("===================================================");
    println!("============ Area of a circle program In Rust ==============");
    let area_circle = area(20, 12);
    println!("Area of circle: {}", area_circle);
    println!("===================================================");
    println!("===================================================");
    println!("============ Area of a circle with tuple In Rust ==============");
    let area_circle_tuple = area_with_tuple((50, 20));
    println!("Area of circle: {}", area_circle_tuple);
    println!("===================================================");
    println!("===================================================");

    println!("============ Ownership of Struct Data In Rust ==============");
    let react1 = Rectangle {
        width: 100,
        height: 200,
    };
    let area_circle_struct = area_with_struct(&react1);
    println!("Area of circle: {}", area_circle_struct);
    println!("===================================================");
    println!("===================================================");
    println!("======================= Methods in Rust ============================");
    let react2 = RectangleM {
        width: 30,
        height: 70,
    };
    let res_area = react2.area();
    println!("Area of circle: {}", res_area);
    println!("===================================================");
    println!("===================================================");
    println!("====================== Enums in Rust =============================");
    enums_in_rust::enums_exam();
    println!("===================================================");
    println!("===================================================");

    println!("======================= Common Collections in Rust============================");
    println!("===================================================");
    println!("===================================================");
    common_collections_in_rs();

    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("=========================== Error handling in Rust ========================");

    println!("===================================================");
    handling_errors_in_rs();

    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("============= Generic Types, Traits, and Lifetimes =================");

    println!("===================================================");
    generics_and_traits_in_rs();
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("====================== Closure in Rust  ===================");
    closures_in_rs();

    println!("===================================================");
    traits_in_rs();
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("====================== Smart Pointer in Rust  ===================");
    smart_pointers();

    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("====================== ConCurrency in Rust  ===================");
    all_concurrency_exam();
    messages_passing_trn();
    multiple_messages_tr();

    println!("===================================================");
    println!("===================================================");
    println!("==================== OOP in rust ===================");
    println!("===================================================");
    object_orient_program_rs();
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
    println!("==================== Unsafe rust ===================");
    println!("===================================================");
    unsafe_rs::unsafe_in_rs();
    println!("===================================================");
    println!("===================================================");
    println!("===================================================");
}

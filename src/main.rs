use ::std::cmp::Ordering;
use rand::Rng;
use std::io;
mod ownership_in_rust;
mod references_borrowing;
mod structures_in_rust;
mod varials_in_rs;

use ownership_in_rust::ownership_rs_tuto;
use references_borrowing::references_main_func;
use structures_in_rust::structures_in_rust;
use varials_in_rs::variables_in_rust;

mod area_of_circle;

use area_of_circle::area;

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
    variables_in_rust();
    println!("============ OwnerShip In Rust ==============");
    ownership_rs_tuto();
    println!("============ References and Borrowing In Rust ==============");
    references_main_func();
    println!("============ Struct / Structures In Rust ==============");
    structures_in_rust();
    println!("============ Area of a circle program In Rust ==============");
    let area_circle = area(20, 12);
    println!("Area of circle: {}", area_circle);
}

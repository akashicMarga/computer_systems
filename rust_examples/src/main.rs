mod hello; // This line imports the hello module

use std::env;

fn main() {
    let program = env::args().nth(1).expect("No program specified");
    match program.as_str() {
        "hello" => hello::hello(), // Calls the hello function from the hello module
        _ => println!("Unknown program: {}", program),
    }
}


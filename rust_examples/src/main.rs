mod hello; // This line imports the hello module
mod byte_rep; // This line imports the byte_rep module

use std::env;

fn main() {
    let program = env::args().nth(1).expect("No program specified");
    match program.as_str() {
        "hello" => hello::hello(), // Calls the hello function from the hello module
        "byte_rep" => byte_rep::test_show_bytes(12345), // Calls the test_show_bytes function from the byte_rep module
        _ => println!("Unknown program: {}", program),
    }
}


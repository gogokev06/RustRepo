//-----------------------//
// Simplest wat to get user input (different line as prompt)//
use std::io;

// Function: Takes in a prompt and outputs the response.
fn get_input(prompt: &str) -> String {
    println!("{}", prompt); // Prints prompt.
    let mut input: String = String::new(); // Creates new empty line.
    io::stdin()
        .read_line(&mut input) // Changes and references the empty line to the user's input.
        .expect("Failed to read line!"); // Fail-safe system.
    input.trim().to_string() // Converts the &str type input into String type.
}
//-----------------------//
// Best wat to get user input (same line as prompt)//
use std::io::{self, Write};

fn get_input_alt(prompt: &str) -> String {
    print!("{}", prompt); // The print! macro is for no new line
    io::stdout() // Extra part for ensuring same-line reading command.
        .flush()
        .expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input.trim().to_string()
}
//-----------------------//
// Main function.
fn main() {
    let name: String = get_input("State your name: ");
    println!("Welcome {}", name)

    let name: String = get_input_alt("State your name: ");
    println!("Welcome {}", name)
}


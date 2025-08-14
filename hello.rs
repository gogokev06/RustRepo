use std::io;

// Function: Takes in a prompt and outputs the response.
fn get_input(prompt: &str) -> String {
    println!("{}", prompt); // Prints prompt.
    let mut input: String = String::new(); // Creates new empty line.
    io::stdin()
        .read_line(&mut input) // Changes and references the empty line to the user's input.
        .expect("Failed to read line!");
    input.trim().to_string()
}

// Main function.
fn main() {
    let name: String = get_input("State your name: ");
    println!("Welcome {}", name)
}

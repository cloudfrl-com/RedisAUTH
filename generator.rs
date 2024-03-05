// Rust program to generate a random password of 18 ASCII characters
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    // Create a random number generator
    let mut rng = thread_rng();
    // Generate a random password of 18 ASCII characters
    let password: String = rng
        .sample_iter(&Alphanumeric) // Sample from alphanumeric characters
        .take(18) // Take 18 characters
        .map(char::from) // Convert to char
        .collect(); // Collect into a string
    // Print the password
    println!("Generated password: {}", password);
}

// Sample program that prints to console
//
// Still best one around yet!

/// The main function runs when the program starts
/// and typically has access to command line arguments
fn main() {
    print("Hello, world!");
    print("Goodbye, cruel world!")
}

// a function that prints a message
fn print(m: &str){
    println!("{m}")
}

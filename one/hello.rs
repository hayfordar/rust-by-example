// Example macro call
macro_rules! say_hello {
	// macro takes no arguments
	() => (
		println!("macro_rules!")
	)
}

// Main function
fn main() {
	// Simple println! and wrapper macro for println!
	println!("Hello World!");
	say_hello!();
	
	// Printing with substitution	
	println!("\"I just found out {0} can use they {1}\" - Young Thug"
		, "rats", "lips");

	// Print with control on number of decimal places
	println!("{:.4}", 3.14159265358979);

}

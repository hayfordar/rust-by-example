use std::fmt;

// Lifetime of name driven by lifetime of struct
#[derive(Debug)]
struct Person<'a> {
	name: &'a str,
	age: u8
}

// Need to define lifetime of impl, Person
impl<'a> fmt::Display for Person<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{0} is {1}", self.name, self.age)
	}
}

fn main() {
	let name = "Jeffery";
	let age = 26;
	let thugger = Person { name, age };
	
	// Debug print
	println!("{:?}", thugger);

	// Prettier debug print
	println!("{:#?}", thugger);

	// Impl println for Person
	println!("{}", thugger);
}

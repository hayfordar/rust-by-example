use std::fmt;

// Define a custom structure for a vector
struct Vector(Vec<f32>);

impl fmt::Display for Vector {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let v = &self.0;

		write!(f, "[")?;

		for (count,val) in v.iter().enumerate() {
			if count != 0 { write!(f, ", ")?; }
			write!(f, "{0}:{1}", count, val)?;
		}	
		
		write!(f, "]")
	}
}

fn main() {
	let v = Vector (vec![3.14, 6.28, 0.0]);
	println!("{}", v);
}

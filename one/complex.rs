use std::fmt;

#[derive(Debug)]
struct Complex {
	real: f32,
	imag: f32
}


impl fmt::Display for Complex {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{0} + {1}i", self.real, self.imag)
	}
}

fn main() {
	let compl = Complex { real : 31.0, imag : -17.11 };
	println!("{:?}", compl);
	println!("{}", compl);
}

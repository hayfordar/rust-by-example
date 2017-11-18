use std::fmt;

struct Color {
	r : u8,
	g : u8,
	b : u8,
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RGB({0}, {1}, {2}), 0x{0:X}{1:X}{2:X}",
			self.r, self.g, self.b)
	}
}

fn main() {
	let random_color = Color { r: 128, g : 44, b : 91 };
	println!("{}", random_color);
}

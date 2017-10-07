use std::fmt;

#[derive(Debug)]
struct CelestialObject<'a> {
	name: &'a str,	
	decl: f32,		// Declination in respect to celestial equator, deg
	rasc: f32,		// Right ascension from Greenwich meridian, deg
}

// Display CelestialObject coordinates (decl, rasc) in (degrees, sidereal hours)
impl<'a> fmt::Display for CelestialObject<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let decl_ind = if self.decl >= 0.0 { 'N' } else { 'S' };
		let rasc_h = self.rasc / (360.0 / 24.0);
		let rasc_m = (rasc_h % rasc_h.floor()) * 60.0;
		let rasc_s = (rasc_m % rasc_m.floor()) * 60.0;

		write!(f, "{5}: {0}deg {1}, {2}h {3}' {4}\""
			, self.decl, decl_ind, rasc_h.floor(), rasc_m.floor()
			, rasc_s, self.name)
	}
}

fn main() {
	let polaris = CelestialObject { name: "Polaris", decl: 89.25, rasc: 37.9542 };
	println!("{}", polaris);
}

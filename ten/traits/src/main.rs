enum Colors {
    RGB(Rgb),
    CMYK(Cmyk)
}

struct Rgb {
    r : u8,
    g : u8,
    b : u8
}

struct Cmyk {
    c : f32,
    m : f32,
    y : f32,
    k : f32
}

trait ColorAsString {
    fn color_to_string(&self) -> String {
        String::from("Color")
    }
}

impl ColorAsString for Rgb {
    fn color_to_string(&self) -> String {
        format!("R: {}, G: {}, B: {}", self.r, self.g, self.b)
    }
}

impl ColorAsString for Cmyk { }

fn main() {
    let r = 255;
    let g = 255;
    let b = 255;
    let color1 = Rgb { r, g, b };
    println!("{:?}", color1.color_to_string());

    let c = 0.0;
    let m = 0.0;
    let y = 0.0;
    let k = 0.0;
    let color2 = Cmyk { c, m, y, k };
    println!("{:?}", color2.color_to_string());

}

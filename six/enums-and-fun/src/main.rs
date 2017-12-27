use std::cmp::max;

enum ColorEncoding {
    CMYK(Cmyk),
    RGB(Rgb)
}

struct Rgb {
    r : i32,
    g : i32,
    b : i32
}

struct Cmyk {
    c : f32,
    m : f32,
    y : f32,
    k : f32
}

fn change_encode(color : ColorEncoding) {
    match color {
        ColorEncoding::RGB(rgb) => {
            let rp : f32 = rgb.r as f32 / 255.0;
            let gp : f32 = rgb.g as f32 / 255.0;
            let bp : f32 = rgb.b as f32 / 255.0;
        }
        ColorEncoding::CMYK(cmyk) => {
            println!("{:?}", "no");
        }
    }
}

fn main() {
    let color = ColorEncoding::RGB(Rgb { r : 123, g : 33, b : 13 });
    change_encode(color);
}

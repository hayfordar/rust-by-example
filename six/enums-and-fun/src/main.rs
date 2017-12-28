#[derive(Debug)]
enum ColorEncoding {
    CMYK(Cmyk),
    RGB(Rgb)
}

#[derive(Debug)]
struct Rgb {
    r : i32,
    g : i32,
    b : i32
}

#[derive(Debug)]
struct Cmyk {
    c : f32,
    m : f32,
    y : f32,
    k : f32
}

fn change_encode(color : ColorEncoding) -> ColorEncoding {
    match color {
        ColorEncoding::RGB(rgb) => {
            let rp : f32 = rgb.r as f32 / 255.0;
            let gp : f32 = rgb.g as f32 / 255.0;
            let bp : f32 = rgb.b as f32 / 255.0;

            let k = 1.0 - rp.max(gp).max(bp);
            let c = (1.0 - rp - k) / (1.0 - k);
            let m = (1.0 - gp - k) / (1.0 - k);
            let y = (1.0 - bp - k) / (1.0 - k);

            ColorEncoding::CMYK( Cmyk { c, m, y, k })
        }
        ColorEncoding::CMYK(cmyk) => {
            let r : i32 = (255.0 * (1.0 - cmyk.c) * (1.0 - cmyk.k)) as i32;
            let g : i32 = (255.0 * (1.0 - cmyk.m) * (1.0 - cmyk.k)) as i32;
            let b : i32 = (255.0 * (1.0 - cmyk.y) * (1.0 - cmyk.k)) as i32;

            ColorEncoding::RGB( Rgb { r, g, b})
        }
    }
}

fn main() {
    let color = ColorEncoding::RGB(Rgb { r : 123, g : 33, b : 13 });
    println!("{:?}", color);
    let color = change_encode(color);
    println!("{:?}", color);
}

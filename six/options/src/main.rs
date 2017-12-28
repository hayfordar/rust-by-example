fn add(x : &u8, y : &u8) -> Option<u8> {
    // Very naive way to test for overflow, simply an example
    if (*x as u16) + (*y as u16) > 255 {
        None
    } else {
        Some(*x + *y)
    }
}

fn handle(opt : Option<u8>) -> Option<u8> {
    match opt {
        None => {
            println!("Integer overflow!");
            None
        },
        Some(u) => Some(u)
    }
}

fn main() {
    let x : u8 = 128;
    let y : u8 = 128;
    let z : u8 = 1;

    println!("{:?}", handle(add(&x, &y)));
    println!("{:?}", handle(add(&x, &z)));
}

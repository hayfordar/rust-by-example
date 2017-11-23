const DEFINED_CONSTANT : u32 = 100;

fn main() {
    let integer : u32 = 10;
    let float : f32 = 0.0;

    println!("Float: {1:.4}, integer: {0}", integer, float);

    let integer = integer + 1;
    let integer = integer * DEFINED_CONSTANT;

    println!("integer: {} after shadowing", integer);

    let character : char = 'z';
    println!("A character: {}", character);

    let tup : (u32, i32, usize) = (10, -10, 0);
    println!("{:?}", tup);

    /* alias the content of tuple tup */
    let (a, b, c) = tup;

    println!("a : {}, b : {}, c : {}", a, tup.1, c);

    /* arrays are always fixed size, stack allocated objects */
    let array = [10.0, 0.0, 6.6, 0.6];
    println!("array[0] = {}", array[0]);
}

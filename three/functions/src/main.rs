fn print_num(number : u32) {
    println!("{}", number);
}

fn print_two_nums(num1 : u32, num2 : u32) {
    println!("{} {}", num1, num2);
}

fn add_one(x : u32) -> u32 {
    x + 1
}

fn main() {
    let x = 5;
    let y = 6;

    print_num(x);
    print_two_nums(x,y);

    let z = add_one(x);
    print_num(z);
}

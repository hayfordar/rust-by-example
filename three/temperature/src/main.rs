fn main() {
    println!("{}", to_celsius(&32.0));
    println!("{}", to_celsius(&0.0));
    println!("{}", to_celsius(&10.0));
    println!("{}", to_celsius(&20.0));

    println!("{}", to_fahrenheit(&0.0));
    println!("{}", to_fahrenheit(&10.0));
    println!("{}", to_fahrenheit(&20.0));
    println!("{}", to_fahrenheit(&30.0));
}

/* t_c = (t_f - 32.0) * 5.0 / 9.0 */
fn to_celsius(temp_f : &f64) -> f64 {
    (*temp_f - 32.0) * (5.0 / 9.0)
}

/* t_f = (9/5)*t_c + 32.0 */
fn to_fahrenheit(temp_c : &f64) -> f64 {
    (*temp_c * (9.0 / 5.0)) + 32.0
}

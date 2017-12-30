fn main() {
    let literal = "Hello World!";

    let s1 : String = literal.to_string();
    let s2 : String = String::from(literal);

    let mut s3 : String = String::new();
    s3.push_str(&literal);

    // s1, s2, and s3 are all equivalent means of instantiating a String from a string literal
    println!("{:?} {:?} {:?}", s1, s2, s3);

    // Append single character with push method
    s3.push('!');
    println!("{:?}", s3);

    // String concatenation
    let s4 : String = format!("{}, the \" {} \"", "Lil B", "Based God");
    println!("{}", s4);
    let s5 : String = s4 + ", " + &s1;
    println!("{}", s5);

    // Iterate through string as Unicode scalar values
    for c in s5.chars() {
        println!("{}", c);
    }

    // Iterate through the raw bytes of a string
    // Use carefully as characters are not necessarily single byte objects in unicode/UTF-8
    for b in s5.bytes() {
        println!("{}", b);
    }

}

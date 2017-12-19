// Returns string slice containing the first word in the string
fn first_word(s : &str) -> &str {
    let mut i = 0;

    for byte in s.as_bytes() {
        if *byte == b' ' {
            return &s[0..i]
        }
        i = i + 1;
    }

    &s
}

fn main() {
    let mut s1 = String::from("Papa bless yall");
    assert!(first_word(&s1) == "Papa");

    s1 = s1.replace(" ", "");
    assert!(first_word(&s1) == "Papablessyall");
}

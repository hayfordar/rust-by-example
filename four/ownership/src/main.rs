fn main() {
    let s1 = String::from("Initial");
    let s2 = s1;

    // Below will fail because Rust implements `let s2 = s1`
    // as a MOVE operation. s1 is invalidated when s2 takes
    // ownership.
    // println!("{:?}", s1);

    // Shadow s2 back after transferring and receiving ownership
    let s2 = takes_returns_ownership(s2);
    takes_abandons_ownership(s2);

    // Below will fail because ownership was transfered to called
    // function and the variable holding the pointer to memory fell
    // out of scope
    // println!("{:?}", s2);

    let s3 = gives_ownership();
    println!("{:?}", s3);
}

fn takes_abandons_ownership(str : String) {
    println!("Taking and abandoning ownership of passed string");
}

fn takes_returns_ownership(str : String) -> String {
    println!("Taking ownership of passed string, returning");
    str
}

fn gives_ownership() -> String {
    println!("Giving ownership to a string created in `gives_ownership`");
    String::from("Created string")
}

use std::collections::HashMap;

fn main() {
    let mut scores : HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("McGill"), 0);
    scores.insert(String::from("Andrew"), 6);
    scores.insert(String::from("Samson"), 10);

    println!("Print full content:");
    println!("{:?}\n", scores);

    println!("Individual item access:");
    println!("{:?}\n", scores.get("Andrew"));

    // Update or insert
    scores.entry(String::from("Andrew")).or_insert(7);
    scores.entry(String::from("Delilah")).or_insert(6);

    println!("After or_insert for Andrew and Delilah");
    println!("{:?}\n", scores);    
}

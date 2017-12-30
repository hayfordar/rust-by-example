use std::collections::HashMap;

fn main() {
    let mut albums : HashMap<String, i32> = HashMap::new();
    // Below will also work, types will be inferred based on first key/value pair
    //let mut albums : HashMap = HashMap::new();

    albums.insert(String::from("4eva Is A Mighty Long Time"), 95);
    albums.insert(String::from("Black Ken Mixtape"), 85);
    albums.insert(String::from("Easy, Breezy, Beautiful Thugger Girls"), 65);
    albums.insert(String::from("Jeffery"), 100);

    let artists : Vec<String> = Vec::new();
    Vec.push(String::from("Big K.R.I.T."));
    Vec.push(String::from("Lil B"));
    Vec.push(String::from("Young Thug"));

    


}

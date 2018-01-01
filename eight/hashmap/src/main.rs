use std::collections::HashMap;

fn main() {
    let mut albums : HashMap<String, i32> = HashMap::new();
    // Below will also work, types will be inferred based on first key/value pair
    //let mut albums : HashMap = HashMap::new();

    albums.insert(String::from("4eva Is A Mighty Long Time"), 95);
    albums.insert(String::from("Black Ken Mixtape"), 85);
    albums.insert(String::from("Easy, Breezy, Beautiful Thugger Girls"), 65);
    albums.insert(String::from("Jeffery"), 100);

    let mut artists : Vec<String> = Vec::new();
    artists.push(String::from("Big K.R.I.T."));
    artists.push(String::from("Lil B"));
    artists.push(String::from("Young Thug"));

    let curr_rank : Vec<i32> = vec![3, 5, 4];

    // Get iterator through artists and zip with iterator for rank vector, collect to HashMap
    let mut artist_ranks : HashMap<_, _> = artists.iter().zip(curr_rank.iter()).collect();
    println!("{:?}", artist_ranks);

    // References inserted into a HashMap do not have ownership, we can still do below:
    println!("{:?}", artists);

    // Types that implement copy are copied, ownership is taken for types like Strings
    let earl : String = String::from("Earl Sweatshirt");
    let earl_rank : i32 = 2;

    let mut artist_ranks2 : HashMap<String, i32> = HashMap::new();
    artist_ranks2.insert(earl, earl_rank);

    // Cannot do below as Copy is not implemented, ownership transferred to HashMap
    //println!("{:?}", earl);
    println!("{:?}", earl_rank);

    // Loop through each key, value pair
    for (key, value) in &artist_ranks {
        println!("{:?} {:?}", key, value);
    }
}

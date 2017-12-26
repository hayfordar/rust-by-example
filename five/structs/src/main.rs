#[derive(Debug)]
struct Person {
    based : bool,
    name : String,      // Use of String means 'name' String exists as long as Person
    age : i32
}

// Function to return instance of Person
fn create_person(based : bool, name : String, age : i32) -> Person {
    Person { based
         , name
         , age }
    // Above is shorthand when field names match variables, below req otherwise
    // Person { based : based
    //  , name : name
    //  , age : age }
}

fn main() {
    let mut lil_b = Person { based : true, name : String::from("Lil B the \" Based God \"")
        , age : 29 };
    lil_b.name = String::from("Lil B");
    println!("{:?}", lil_b);

    // We can use the .. syntax to copy uninitialized values from lil_b
    let andrew = Person { name : String::from("Andrew")
        , age: 23 , ..lil_b };
    println!("{:?}", andrew);

    let mut jeffery = create_person(false, String::from("Young Thug"), 27);
    jeffery.name = String::from("Jeffery");
    println!("{:?}", jeffery);
}

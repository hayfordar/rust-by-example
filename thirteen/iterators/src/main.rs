// Iterator for a type that returns a value of that specified type
// Iterator trait only requires you implement next(), which returns
// one element per call, wrapped in Some(), or None when the
// iteration is over
//
// trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
// }

#[derive(Debug, PartialEq)]
enum ShoeStyle {
    HighTop,
    Mid,
    Low,
    Sandal,
    Slide,
    Boot,
    Formal
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size : u32,
    style : ShoeStyle
}

fn shoes_in_style(style : ShoeStyle, shoes : Vec<Shoe>) -> Vec<Shoe> {
    // We can use the filter() method to create a new vector from items meeting criteria
    // in a passed closure.
    shoes.into_iter()
        .filter(|item| item.style == style)
        .collect()
}

fn main() {
    // Example One: Simple iterator in loop
    let vec1 : Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    for item in vec1.iter() {
        println!("{}", item);
    }

    // Example Two: Iterator with mutable access to items in vector
    let mut vec2 : Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for item in vec2.iter_mut() {
        *item = *item + 1;
    }

    println!("{:?}", vec2);

    // Example Three: Explicitly defined iterators are mutable
    let mut iter = vec1.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    // Code 'consumes' iterators such that each call 'removes' an item

    // The call to sum takes ownership of iter, thus it cannot be used after the below:
    let total : u32 = iter.sum();
    println!("{:?}", total);

    // Example Four: Using iterator adapters and closures together:
    // Create a new vector, vec3, by iterating through vec2 and running
    // the closure passed to map() on each element. We need to run collect()
    // in order to consume the iterator and populate vec3, as iterators are lazy
    let vec3 : Vec<u32> = vec2.iter().map(|item| item * 2).collect();
    println!("{:?}", vec3);

    println!("Hello, world!");

    let shoes : Vec<Shoe> = vec![
        Shoe { size : 11, style : ShoeStyle::Mid }
        , Shoe { size : 11, style : ShoeStyle::Mid }
        , Shoe { size : 12, style : ShoeStyle::Low }
        , Shoe { size : 12, style : ShoeStyle::Low }
        , Shoe { size : 13, style : ShoeStyle::Boot }];

    println!("{:?}", shoes_in_style(ShoeStyle::Boot, shoes));
}

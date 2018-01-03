// Limit T type to those which implement Copy and PartialOrd
fn largest<T : PartialOrd + Copy>(list : &[T]) -> Option<T> {
    // get method returns Option<T> where T is a reference
    let mut largest = match list.get(0) {
        // dereference x if a valid reference is returned
        Some(x) => *x,
        None => return None
    };

    // Get immutable reference to each value in slice
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

fn main() {
    let vec1 : Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec2 : Vec<f64> = vec![3.0, 5.5, 6.1, 2.931];

    assert!(largest(&vec1) == Some(5));
    assert!(largest(&vec2) == Some(6.1));
}

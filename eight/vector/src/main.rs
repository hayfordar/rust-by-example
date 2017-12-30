fn check_get(opt : Option<&u32>) -> bool {
    match opt {
        Some(val) => true,
        None => {
            println!("Indexed out of bounds, try again");
            false
        }
    }
}

fn main() {
    let mut vec : Vec<u32> = Vec::new();
    for n in 0..5 {
        vec.push(n);
    }

    // Equivalent to the above
    // let mut vec = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];

    // This would panic
    // let out_of_bounds = vec[7];

    let mut n : usize = 7;

    // Illustrate Option<T> as means for index safety, prefer using Vec::get() over index notation
    loop {
        match check_get(vec.get(n)) {
            true => {
                println!("{:?}", vec.get(n));
                break;
            },
            false => {
                // Decrease index until we find valid value
                n = n - 1;
                continue;
            }
        }
    }

    // Iterate through a vector using an immutable borrow of each element
    for item in &vec {
        println!("{:?}", item);
    }

    // Iterate through a vector, using a mutable borrow on each element
    for item in &mut vec {
        *item *= 10;
    }
}

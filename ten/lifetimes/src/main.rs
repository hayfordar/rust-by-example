struct Df<'a> {
    // reference to samples will exist as long as the parent struct exists
    samples : &'a mut [f32]
}

// function constrained with lifetime 'a, where both arguments have lifetime 'a
// and the return value shares the same lifetime 'a of the inputs
fn larger<'a>(vec1 : &'a [f32], vec2 : &'a [f32]) -> &'a [f32] {
    if vec1.len() > vec2.len() {
        vec1
    } else {
        vec2
    }
}

// This function will not compile, because the definition includes the return of
// a borrowed value. With no lifetime, the return value could theoretically reference
// a value that no longer exists
//
// fn larger_no_lifetime(vec1 : & [f32], vec2 : & [f32]) -> & [f32] {
//     if vec1.len() > vec2.len() {
//         vec1
//     } else {
//         vec2
//     }
// }

fn main() {
    let vec1 : Vec<f32> = vec![1.0, 2.0, 3.0];
    let mut vec2 : Vec<f32> = vec![4.0, 5.0, 6.0, 7.0];

    println!("{:?}", larger(&vec1, &vec2));

    {
        // In doing this, we can no longer use the push method, as we have taken a mutable borrow
        let newdf = Df { samples : &mut vec2 };
        match newdf.samples.get_mut(0) {
            Some(x) => {
                *x = 10.0;
            },
            None => {
                println!("Nah dawg");
            }
        }
    }

    println!("{:?}", vec2);
    vec2.push(6.0);
    println!("{:?}", vec2);

    // println!("{:?}", larger_no_lifetime(vec1, vec2));
}

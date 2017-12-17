extern crate crossbeam;

fn main() {
    let num_chunks : usize = 4;
    let chunksize : usize = 4;

    let mut vec : Vec<usize> = Vec::new();
    vec.reserve(num_chunks * chunksize);

    for n in 0..(num_chunks * chunksize) {
        vec.push(n);
    }

    println!("{:?}", vec);

    crossbeam::scope(|scope| {
        for slice in vec.chunks_mut(chunksize) {
                scope.spawn(move || handle_slice(slice));
        }
    });

    println!("{:?}", vec);
}

fn handle_slice(slice : &mut [usize]) {
    for n in 0..slice.len() {
        slice[n] = slice[n] + 1;
    }
}

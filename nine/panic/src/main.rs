use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("Oh shit!!!!");

    let filename = "/home/six/test.txt";

    let file = File::open(filename);

    // Return values of functions can use Result<T>
    // enum Result<T,E> {
    //     Ok(T),
    //     Err(E)
    // }

    // We can match on the return value of a Result<T, E>
    // to either panic or continue with a valid file handle
    let file = match file {
        Ok(file) => file,

        // This is a naive approach, maybe we want to create the file if
        // it does not exist?
        // Err(error) => {
        //     panic!("Failed to open file: {}.", error);
        // }

        Err(error) => match error.kind() {
            // If file is not found, try to create it
            ErrorKind::NotFound => {
                match File::create(filename) {
                    Ok(file) => file,
                    Err(error) => {
                        panic!("Failed to open file: {}", error);
                    }
                }
            },
            // Error is not a 'File Not Found' error, panic!
            _ => {
                panic!("Failed to open file: {}", error);
            }
        }
    };

    // We can use unwrap() or expect(message : str) methods on a Result<T, E>
    // to return the value if Ok<T>, panic! if Err<E>
}

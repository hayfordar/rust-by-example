use std::env;
use std::fs::File;
use std::io::ErrorKind;

struct Config<'a> {
    file : File,
    pattern : &'a str
}

impl<'a> Config<'a> {
    fn new(args : &'a [String]) -> Result<Config, &'a str> {
        if args.len() < 3 {
            return Err("Usage: `cargo run rusty-grep filename \"pattern\"`")
        }

        let filename = match args.get(1) {
            Some(val) => val,
            None => {
                return Err("Did not receive filename argument")
            }
        };

        let file = match open_or_create_file(filename) {
                Ok(file) => file,
                Err(err) => {
                    return Err("Could not open or create requested file")
                }
        };

        let pattern = match args.get(2) {
            Some(val) => val,
            None => {
                return Err("Did not receive argument for pattern")
            }
        };

        Ok(Config { file : file, pattern : pattern })
    }
}

fn open_or_create_file(filename : &str) -> Result<File, std::io::Error> {
    match File::open(filename) {
        Ok(file) => Ok(file),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                match File::create(filename) {
                    Ok(file) => Ok(file),
                    Err(err) => Err(err)
                }
            },
            _ => Err(err)
        }
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let conf = match Config::new(&args) {
        Ok(conf) => conf,
        Err(err) => panic!("{}", err)
    };

    println!("filename: {:?}, pattern: {:?}", conf.file, conf.pattern);
}

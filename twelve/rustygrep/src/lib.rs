use std::fs::File;
use std::error::Error;
use std::io::Read;

pub struct Config {
    pub filename : String,
    pub pattern : String
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Usage: `cargo run rusty-grep filename \"pattern\"`")
        }

        let filename = match args.get(1) {
            Some(val) => val,
            None => {
                return Err("Did not receive filename argument")
            }
        };

        let pattern = match args.get(2) {
            Some(val) => val,
            None => {
                return Err("Did not receive argument for pattern")
            }
        };

        Ok(Config { filename : filename.clone(), pattern : pattern.clone() })
    }

    fn open_file(filename : &str) -> Result<File, std::io::Error> {
        match File::open(filename) {
            Ok(file) => Ok(file),
            Err(err) => Err(err)
        }
    }
}

pub fn run(config : &Config) -> Result<(), Box<Error>> {
    let mut contents = String::new();

    let mut file = Config::open_file(&config.filename)?;
    file.read_to_string(&mut contents)?;

    println!("{:?}", contents);

    Ok(())
}

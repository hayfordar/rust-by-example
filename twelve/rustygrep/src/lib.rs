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

    for line in search(&config.pattern, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query : &str, content : &'a str) -> Vec<&'a str> {
    let mut results : Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_match() {
        let query = "six";
        let content = "\
I don't mean to intimidate you
but my high score
is six.";
        assert_eq!(vec!["is six."]
            , search(query, content));
    }

    #[test]
    fn multiple_match() {
        let query = "he ";
        let content = "\
villain man never ran with krills in his hand
and, won't stop rockin til he clocked in a gazillion grand,
tillin the wasteland sands,
raps on backs of treasure maps, stacks to the ceiling fan
he rest when he's ashes
ask him after ten miles in his goulashes, smashes stashes";
        assert_eq!(vec!["and, won't stop rockin til he clocked in a gazillion grand,"
            , "tillin the wasteland sands,"
            , "raps on backs of treasure maps, stacks to the ceiling fan"
            , "he rest when he's ashes"],
            search(&query, &content));
    }
}

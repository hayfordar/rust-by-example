use std::env;
use std::process;

extern crate rustygrep;
use rustygrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();

    let conf = match Config::new(&args) {
        Ok(conf) => conf,
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        }
    };

    println!("filename: {:?}, pattern: {:?}", conf.filename, conf.pattern);
}

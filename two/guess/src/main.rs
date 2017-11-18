extern crate rand;

use std::io;
use std::cmp;
use rand::Rng;

fn main() {
	loop {
		println!("Provide a number:");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");

		println!("You guessed {}", guess);

		/* Shadow guess variable name and parse out a u32 */
		let guess : u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		let actual = rand::thread_rng().gen_range(1,10);

		match guess.cmp(&actual) {
			cmp::Ordering::Less => println!("Failure: Number was {}", actual),
			cmp::Ordering::Greater => println!("Failure: Number was {}", actual),
			cmp::Ordering::Equal => {
				 println!("You win!");
				 break;
			}
		}
	}
}

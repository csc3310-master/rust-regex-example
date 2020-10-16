use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() {
	println!("Enter a string: ");
	let mut input = read_line_iter();
	println!("Your input: {}", input);
	let myregex = r"\d\d\d[a-z]+$";
	let re = Regex::new(myregex).unwrap();

	while re.is_match(&input) {
		println!("It matched {}", input );
		println!("Enter a string: ");
		input = read_line_iter();

	}
	println!("It is not a match");
	
}
fn read_line_iter() -> String{
	let stdin = io::stdin();
	let input = stdin.lock().lines().next();
	input
		.expect("No lines in buffer")
		.expect("Failed to read line")
		.trim()
		.to_string()
	
}

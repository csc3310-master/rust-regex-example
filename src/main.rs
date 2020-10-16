use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() {
	println!("Enter a string: ");
	let input = read_line_iter();
	println!("Your input: {}", input);
	let re = Regex::new(r"\d\d\d[a-z]+$").unwrap();
	if re.is_match(&input) {
		println!("It matched!");
	}else{
		println!("It is not a match");
	}
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

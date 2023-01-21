// Hunter Westerlund
// 01/20/23
/*
	This is my implementation of the Minigrep project
*/

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	let query =     &args[1];
	let file_path = &args[2];

	println!("Searching for {}", query);
	println!("In file {}", file_path);
}

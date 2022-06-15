use debrixc::build;
use std::io::{self, Read};

fn main() {
	let mut input = String::new();
    let stdin = io::stdin();
	let mut handle = stdin.lock();
	handle.read_to_string(&mut input).unwrap();
	let result = build(&input);
	println!("{}", result.source);
}

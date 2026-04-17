mod printer;

use std::io::{self, Read};

fn main() {
	let mut input = Vec::new();
	io::stdin().read_to_end(&mut input).unwrap();

	let html = printer::ansi_to_html(&input);
	println!("{}", html);
}

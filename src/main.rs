mod printer;

use std::io::{self, Read};

// reset, truecolor, bold, italic, underline, and background in ISO 8613-6 / extended SGR form + newline
// printf "\033[0;38:2:1:30:144:255;1;3;4;48:2:127:0:127mhello\033[0m\n" | cargo run
fn main() {
	env_logger::init();

	let mut input = Vec::new();
	io::stdin().read_to_end(&mut input).unwrap();

	let html = printer::ansi_to_html(&input);
	println!("{}", html);
}

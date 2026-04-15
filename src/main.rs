use std::io::{self, Read};
use vte::{Parser, Perform};

struct Printer;

impl Perform for Printer {
	fn print(&mut self, c: char) {
		println!("print: {:?}", c);
	}

	fn execute(&mut self, byte: u8) {
		println!("execute: 0x{:x}", byte);
	}

	fn csi_dispatch(
		&mut self,
		params: &vte::Params,
		intermediates: &[u8],
		ignore: bool,
		action: char,
	) {
		println!(
			"CSI: params={:?}, intermediates={:?}, ignore={}, action={:?}",
			params, intermediates, ignore, action
		);
	}
}

fn main() {
	let mut parser = Parser::new();
	let mut performer = Printer;

	let mut input = Vec::new();
	io::stdin().read_to_end(&mut input).unwrap();

	for byte in input {
		parser.advance(&mut performer, byte);
	}
}

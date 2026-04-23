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

		if action != 'm' {
			return;
		}

		for p in params.iter() {
			println!("param bytes: {:02x?}", p);

			match p {
				[0x00] => println!("reset 0x00 (0)"),
				[0x01] => println!("bold 0x01 (1)"),
				[0x03] => println!("italic 0x03 (3)"),
				[0x04] => println!("underline 0x04 (4)"),
				[0x26, _rest @ ..] => println!("color 0x26 (38)"),
				[0x30, _rest @ ..] => println!("background 0x30 (48)"),
				_ => {
					println!("anything else");
				}
			}
		}
	}
}

pub fn ansi_to_html(input: &[u8]) -> String {
	let mut parser = Parser::new();
	let mut printer = Printer;

	for &byte in input {
		parser.advance(&mut printer, byte);
	}

	String::new()
}

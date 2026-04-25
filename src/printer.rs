mod ansi_to_css;

use ansi_to_css::Converter;
use vte::{Parser, Perform};

struct Printer {
	converter: Converter,
}

impl Printer {
	pub fn new() -> Self {
		Self {
			converter: Converter::new(),
		}
	}
}

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

		self.converter.sgr_params_to_css(params.iter());
	}
}

pub fn ansi_to_html(input: &[u8]) -> String {
	let mut parser = Parser::new();
	let mut printer = Printer::new();

	for &byte in input {
		parser.advance(&mut printer, byte);
	}

	String::new()
}

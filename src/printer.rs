mod ansi_to_css;

use ansi_to_css::Converter;
use vte::{Parser, Perform};

struct Printer {
	converter: Converter,
	output: String,
}

impl Printer {
	pub fn new() -> Self {
		Self {
			converter: Converter::new(),
			output: String::new(),
		}
	}
}

impl Perform for Printer {
	fn print(&mut self, c: char) {
		log::debug!("char={:?}, codepoint=U+{:04X}", c, c as u32);
		if let Some(esc) = escape_html(c) {
			self.output.push_str(esc);
		} else {
			self.output.push(c);
		}
	}

	fn execute(&mut self, byte: u8) {
		log::debug!("execute: 0x{:x}", byte);
		match byte {
			// b'\n' => self.output.push_str("<br>"),
			b'\n' => self.output.push_str("\n"),
			b'\r' => {}
			_ => {}
		}
	}

	fn csi_dispatch(
		&mut self,
		params: &vte::Params,
		intermediates: &[u8],
		ignore: bool,
		action: char,
	) {
		log::debug!(
			"CSI: params={:?}, intermediates={:?}, ignore={}, action={:?}",
			params,
			intermediates,
			ignore,
			action
		);

		if action != 'm' {
			return;
		}

		self.output
			.push_str(&self.converter.sgr_params_to_css(params.iter()));
	}
}

pub fn ansi_to_html(input: &[u8]) -> String {
	let mut parser = Parser::new();
	let mut printer = Printer::new();

	for &byte in input {
		parser.advance(&mut printer, byte);
	}

	log::debug!("output = {:?}", printer.output);
	log::debug!("output bytes = {:02X?}", printer.output.as_bytes());

	printer.output
}

fn escape_html(c: char) -> Option<&'static str> {
	match c {
		'&' => Some("&amp;"),
		'<' => Some("&lt;"),
		'>' => Some("&gt;"),
		'"' => Some("&quot;"),
		'\'' => Some("&#39;"),
		_ => None,
	}
}

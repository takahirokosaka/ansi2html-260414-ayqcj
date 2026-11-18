mod util;

use self::util::style_to_css;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
	Rgb(u8, u8, u8),
	Ansi256(u8),
	None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Style {
	fg: Color,
	bg: Color,
	bold: bool,
	italic: bool,
	underline: bool,
	reverse: bool,
}

impl Default for Style {
	fn default() -> Self {
		Self {
			fg: Color::None,
			bg: Color::None,
			bold: false,
			italic: false,
			underline: false,
			reverse: false,
		}
	}
}

pub(super) struct Converter {
	cur_style: Style,
	open_span: bool,
	output: String,
}

impl Converter {
	pub fn new() -> Self {
		Self {
			cur_style: Style::default(),
			open_span: false,
			output: String::new(),
		}
	}

	pub(super) fn sgr_params_to_css<'a>(
		&mut self,
		params: impl IntoIterator<Item = &'a [u16]>,
	) -> String {
		let mut accumulated = Style::default();

		for p in params {
			log::debug!("param bytes: {:02x?}", p);

			match p {
				[0x0000u16] => {
					log::debug!("reset 0x00 (0)");

					// Reset: close the current span (if any), and open a new span with the default style if the current span is not already the default.
					let new = Style::default();
					self.flush_span_if_needed(&new);
				}
				[0x0001u16] => {
					log::debug!("bold 0x01 (1)");
					accumulated.bold = true;
				}
				[0x0003u16] => {
					log::debug!("italic 0x03 (3)");
					accumulated.italic = true;
				}
				[0x0004u16] => {
					log::debug!("underline 0x04 (4)");
					accumulated.underline = true;
				}
				[0x0007u16] => {
					log::debug!("reverse 0x07 (7)");
					accumulated.reverse = true;
				}
				[0x0026u16, mode, rest @ ..] => {
					log::debug!("{:?}", p);

					match mode {
						2 => {
							log::debug!("truecolor");

							if rest.len() >= 4 {
								let (r, g, b) = (rest[1] as u8, rest[2] as u8, rest[3] as u8);
								accumulated.fg = Color::Rgb(r, g, b);
							} else if rest.len() >= 3 {
								let (r, g, b) = (rest[0] as u8, rest[1] as u8, rest[2] as u8);
								accumulated.fg = Color::Rgb(r, g, b);
							} else {
								log::warn!("truecolor sequence missing RGB values: {:?}", rest);
								accumulated.fg = Color::None;
							}
							log::debug!("foreground truecolor {:?})", accumulated.fg);
						}
						5 => {
							log::debug!("ansi256color");

							// Extract the first element of "rest" safely
							if let [index, ..] = rest {
								accumulated.fg = Color::Ansi256(*index as u8);
								log::debug!("foreground ansi256 index {}", index);
							} else {
								log::warn!("ansi256color sequence missing index: {:?}", rest);
							}
						}
						other => {
							log::warn!("unknown color mode {} rest: {:?}", other, rest);
						}
					}
				}
				[0x0030u16, mode, rest @ ..] => {
					log::debug!("background 0x30 (48)");

					match mode {
						2 => {
							// truecolor: either [extra, R, G, B] or [R, G, B]
							if rest.len() >= 4 {
								let (r, g, b) = (rest[1] as u8, rest[2] as u8, rest[3] as u8);
								accumulated.bg = Color::Rgb(r, g, b);
							} else if rest.len() >= 3 {
								let (r, g, b) = (rest[0] as u8, rest[1] as u8, rest[2] as u8);
								accumulated.bg = Color::Rgb(r, g, b);
							} else {
								log::warn!(
									"background truecolor sequence missing RGB values: {:?}",
									rest
								);
								accumulated.bg = Color::None;
							}
							log::debug!("background truecolor {:?})", accumulated.bg);
						}
						5 => {
							// 256-color: expect single index as first element of rest
							if let [index, ..] = rest {
								accumulated.bg = Color::Ansi256(*index as u8);
								log::debug!("background ansi256 index {}", index);
							} else {
								log::warn!("background ansi256 sequence missing index: {:?}", rest);
							}
						}
						other => {
							log::warn!("unknown background color mode {} rest: {:?}", other, rest);
						}
					}
				}
				[n] if (90..=97).contains(n) => {
					// 90–97 Set bright foreground color
					// Not standard but iTerm2 output these
					log::debug!("bright foreground {}", n);
					let bright_idx = (8 + (*n - 90)) as u8; // 8..=15
					accumulated.fg = Color::Ansi256(bright_idx);
				}
				[n] if (100..=107).contains(n) => {
					// 100–107 Set bright background color
					log::debug!("bright background {}", n);
					let bright_idx = (8 + (*n - 100)) as u8; // 8..=15
					accumulated.bg = Color::Ansi256(bright_idx);
				}
				[n] if (30..=37).contains(n) => {
					// foreground ANSI 8-color
					let idx = (n - 30) as u8; // 32 -> 2
					accumulated.fg = Color::Ansi256(idx);
				}
				[n] if (40..=47).contains(n) => {
					// background ANSI 8-color
					let idx = (n - 40) as u8; // 42 -> 2
					accumulated.bg = Color::Ansi256(idx);
				}
				_ => {
					log::debug!("anything else");
				}
			}
		}

		self.flush_span_if_needed(&accumulated);

		std::mem::take(&mut self.output)
	}

	fn flush_span_if_needed(&mut self, new_style: &Style) {
		if !self.open_span {
			// open a new span for non-default style
			if *new_style != Style::default() {
				log::debug!("style changed to {:?}", new_style);
				if let Some(css) = style_to_css(new_style) {
					self.output.push_str(&format!("<span style=\"{}\">", css));
				} else {
					self.output.push_str("<span>");
				}

				self.open_span = true;
			}
		} else {
			// if style changed, close and open new if needed
			if &self.cur_style != new_style {
				self.output.push_str("</span>");
				self.open_span = false;
				if *new_style != Style::default() {
					log::debug!("style changed to {:?}", new_style);
					if let Some(css) = style_to_css(new_style) {
						self.output.push_str(&format!("<span style=\"{}\">", css));
					} else {
						self.output.push_str("<span>");
					}
					self.open_span = true;
				}
			}
		}
		self.cur_style = *new_style;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// \033[0;38:2:1:30:144:255;1;3;4;48:2:127:0:127m
	#[test]
	fn test_sgr_params_to_css_complex_sequence() {
		let mut converter = Converter::new();

		let out = converter.sgr_params_to_css([
			&[0x0000u16][..],
			&[
				0x0026u16, 0x0002u16, 0x0001u16, 0x001eu16, 0x0090u16, 0x00ffu16,
			],
			&[0x0001u16],
			&[0x0003u16],
			&[0x0004u16],
			&[0x0030u16, 0x0002u16, 0x007fu16, 0x0000u16, 0x007fu16],
		]);

		assert_eq!(
			out,
			"<span style=\"color:#1e90ff; background-color:#7f007f; font-weight:700; font-style:italic; text-decoration:underline;\">"
		);

		let mut converter = Converter::new();

		let out = converter.sgr_params_to_css([
			&[0u16][..],                                 // 0
			&[38u16, 2u16, 1u16, 30u16, 144u16, 255u16], // 38:2:1:30:144:255 -> 38 (set foreground), 2 (truecolor), 1 (? iTerm2 add this, see below), 30 (R), 144 (G), 255 (B)
			&[1u16],                                     // 1 (bold)
			&[3u16],                                     // 3 (italic)
			&[4u16],                                     // 4 (underline)
			&[48u16, 2u16, 127u16, 0u16, 127u16], // 48:2:127:0:127 -> 48 (set background color), 2 (truecolor), R = 127, G = 0, B = 127
		]);

		// The 1 in 38:2:1:30:144:255 is iTerm2's extended RGB syntax. It's an optional color space identifier (where 1 typically means sRGB, the standard). You can safely ignore it for most use cases, or handle it explicitly.

		assert_eq!(
			out,
			"<span style=\"color:#1e90ff; background-color:#7f007f; font-weight:700; font-style:italic; text-decoration:underline;\">"
		);
	}

	#[test]
	fn test_sgr_params_to_css_ansi256_sequence() {
		let mut converter = Converter::new();

		// sequence:
		// 0 (reset)
		// 38:5:196 -> set foreground to ansi256 index 196 (bright red)
		// 1 -> bold
		// 48:5:57  -> set background to ansi256 index 57 (an arbitrary color)
		let out = converter.sgr_params_to_css([
			&[0u16][..],
			&[38u16, 5u16, 196u16][..],
			&[1u16][..],
			&[48u16, 5u16, 57u16][..],
		]);

		assert_eq!(
			out,
			"<span style=\"color:#ff0000; background-color:#5f00ff; font-weight:700;\">"
		);

		let mut converter = Converter::new();

		// fg: Ansi256(12) (bright blue, but not actually bright blue in Dracula)
		let out = converter.sgr_params_to_css([
			&[0u16][..],
			&[38u16, 5u16, 12u16][..],
			&[1u16][..],
			&[48u16, 5u16, 57u16][..],
		]);

		assert_eq!(
			out,
			"<span style=\"color:var(--ansi-bright-blue); background-color:#5f00ff; font-weight:700;\">"
		);
	}

	#[test]
	fn test_sgr_params_to_css_foreground_ansi_32() {
		let mut converter = Converter::new();

		let out = converter.sgr_params_to_css([
			&[0x0000u16][..], // reset
			&[0x0020u16],     // 32 -> foreground index 2 (green)
		]);

		assert_eq!(out, "<span style=\"color:var(--ansi-green);\">");
	}

	#[test]
	fn style_equality_same_values() {
		let s1 = Style {
			fg: Color::Rgb(10, 20, 30),
			bg: Color::Ansi256(42),
			bold: true,
			italic: false,
			underline: true,
			reverse: false,
		};
		let s2 = Style { ..s1 }; // copy fields (Style derives PartialEq + Copy for Color)
		assert_eq!(s1, s2);
		assert!(s1 == s2);
	}

	#[test]
	fn style_equality_different_fg() {
		let s1 = Style {
			fg: Color::Rgb(10, 20, 30),
			bg: Color::Ansi256(42),
			bold: true,
			italic: false,
			underline: true,
			reverse: false,
		};
		let mut s2 = s1;
		s2.fg = Color::Rgb(1, 2, 3);
		assert_ne!(s1, s2);
		assert!(s1 != s2);
	}

	#[test]
	fn style_equality_different_flags() {
		let s1 = Style {
			fg: Color::None,
			bg: Color::None,
			bold: false,
			italic: false,
			underline: false,
			reverse: false,
		};
		let mut s2 = s1;
		s2.bold = true;
		assert_ne!(s1, s2);
	}

	fn style(fg: Color, bg: Color, bold: bool) -> Style {
		Style {
			fg,
			bg,
			bold,
			italic: false,
			underline: false,
			reverse: false,
		}
	}

	#[test]
	fn opens_span_when_current_closed_and_new_non_default() {
		let mut conv = Converter::new();

		let new = style(Color::Rgb(1, 2, 3), Color::None, false);
		conv.flush_span_if_needed(&new);

		assert_eq!(
			conv.open_span, true,
			"should open when new style is non-default"
		);
		assert_eq!(conv.cur_style, new);
	}

	#[test]
	fn stays_closed_when_current_closed_and_new_default() {
		let mut conv = Converter::new();

		let new = Style::default();
		conv.flush_span_if_needed(&new);

		assert_eq!(
			conv.open_span, false,
			"should remain closed for default style"
		);
		assert_eq!(conv.cur_style, new);
	}

	#[test]
	fn closes_and_opens_when_style_changes_between_non_default_styles() {
		let mut conv = Converter {
			cur_style: style(Color::Rgb(1, 1, 1), Color::None, false),
			open_span: true,
			output: String::new(),
		};

		let new = style(Color::Rgb(2, 2, 2), Color::None, false);
		conv.flush_span_if_needed(&new);

		assert_eq!(
			conv.open_span, true,
			"should reopen when changing between non-default styles"
		);
		assert_eq!(conv.cur_style, new);
	}

	#[test]
	fn closes_when_changed_to_default() {
		let mut conv = Converter {
			cur_style: style(Color::Rgb(1, 1, 1), Color::None, false),
			open_span: true,
			output: String::new(),
		};

		let new = Style::default();
		conv.flush_span_if_needed(&new);

		assert_eq!(
			conv.open_span, false,
			"should close when changing to default style"
		);
		assert_eq!(conv.cur_style, new);
	}

	#[test]
	fn stays_open_when_same_style() {
		let existing = style(Color::Rgb(5, 6, 7), Color::None, true);
		let mut conv = Converter {
			cur_style: existing,
			open_span: true,
			output: String::new(),
		};

		let new = existing;
		conv.flush_span_if_needed(&new);

		assert_eq!(
			conv.open_span, true,
			"should remain open when style unchanged"
		);
		assert_eq!(conv.cur_style, new);
	}

	#[test]
	fn stays_closed_when_same_default_style() {
		let mut conv = Converter::new();

		let new = Style::default();
		conv.flush_span_if_needed(&new);

		assert_eq!(conv.open_span, false);
		assert_eq!(conv.cur_style, new);
	}

}

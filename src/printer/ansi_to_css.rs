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
}

impl Converter {
	pub fn new() -> Self {
		Self {
			cur_style: Style::default(),
			open_span: false,
		}
	}

	pub(super) fn sgr_params_to_css<'a>(
		&self,
		params: impl IntoIterator<Item = &'a [u16]>,
	) -> String {
		for p in params {
			println!("param bytes: {:02x?}", p);

			match p {
				[0x0000u16] => {
					println!("reset 0x00 (0)");

					// Reset: close the current span (if any), and open a new span with the default style if the current span is not already the default.
				}
				[0x0001u16] => println!("bold 0x01 (1)"),
				[0x0003u16] => println!("italic 0x03 (3)"),
				[0x0004u16] => println!("underline 0x04 (4)"),
				[0x0007u16] => println!("reverse 0x07 (7)"),
				[0x0026u16, _rest @ ..] => println!("color 0x26 (38)"),
				[0x0030u16, _rest @ ..] => println!("background 0x30 (48)"),
				_ => {
					println!("anything else");
				}
			}
		}

		String::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// \033[0;38:2:1:30:144:255;1;3;4;48:2:127:0:127m
	#[test]
	fn test_sgr_params_to_css_complex_sequence() {
		let converter = Converter::new();

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

		assert!(out.is_empty());

		let out = converter.sgr_params_to_css([
			&[0u16][..],                                 // 0
			&[38u16, 2u16, 1u16, 30u16, 144u16, 255u16], // 38:2:1:30:144:255 -> 38 (set foreground), 2 (truecolor), 1 (? iTerm2 add this), 30 (R), 144 (G), 255 (B)
			&[1u16],                                     // 1 (bold)
			&[3u16],                                     // 3 (italic)
			&[4u16],                                     // 4 (underline)
			&[48u16, 2u16, 127u16, 0u16, 127u16], // 48:2:127:0:127 -> 48 (set background color), 2 (truecolor), R = 127, G = 0, B = 127
		]);

		assert!(out.is_empty());
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
}

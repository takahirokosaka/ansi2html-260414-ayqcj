pub(super) fn sgr_params_to_css<'a>(params: impl IntoIterator<Item = &'a [u16]>) -> String {
	for p in params {
		println!("param bytes: {:02x?}", p);

		match p {
			[0x00] => println!("reset 0x00 (0)"),
			[0x01] => println!("bold 0x01 (1)"),
			[0x03] => println!("italic 0x03 (3)"),
			[0x04] => println!("underline 0x04 (4)"),
			[0x07] => println!("reverse 0x07 (7)"),
			[0x26, _rest @ ..] => println!("color 0x26 (38)"),
			[0x30, _rest @ ..] => println!("background 0x30 (48)"),
			_ => {
				println!("anything else");
			}
		}
	}

	String::new()
}

#[cfg(test)]
mod tests {
	use super::sgr_params_to_css;

	// \033[0;38:2:1:30:144:255;1;3;4;48:2:127:0:127m
	#[test]
	fn test_sgr_params_to_css_complex_sequence() {
		let out = sgr_params_to_css([
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

		let out = sgr_params_to_css([
			&[0u16][..],                                 // 0
			&[38u16, 2u16, 1u16, 30u16, 144u16, 255u16], // 38:2:1:30:144:255 -> 38 (set foreground), 2 (truecolor), 1 (? iTerm2 add this), 30 (R), 144 (G), 255 (B)
			&[1u16],                                     // 1 (bold)
			&[3u16],                                     // 3 (italic)
			&[4u16],                                     // 4 (underline)
			&[48u16, 2u16, 127u16, 0u16, 127u16], // 48:2:127:0:127 -> 48 (set background color), 2 (truecolor), R = 127, G = 0, B = 127
		]);

		assert!(out.is_empty());
	}
}

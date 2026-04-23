pub(super) fn sgr_params_to_css(params: &vte::Params) -> String {
	for p in params.iter() {
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

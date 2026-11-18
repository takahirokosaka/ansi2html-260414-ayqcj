use crate::printer::ansi_to_css::{Color, Style};

// helper: produce "rgb(r,g,b)" string or hex string
fn color_to_css_value(c: Color) -> Option<String> {
	match c {
		Color::Rgb(r, g, b) => Some(rgb_to_hex(r, g, b)),
		Color::Ansi256(idx) => Some(ansi256_to_css_value(idx)),
		Color::None => None,
	}
}

// convert 256-color palette index to (r,g,b)
fn ansi256_to_rgb(idx: u8) -> (u8, u8, u8) {
	let i = idx as u16;
	if i <= 231 {
		let ci = i - 16;
		let r = (ci / 36) % 6;
		let g = (ci / 6) % 6;
		let b = ci % 6;
		let conv = |v| if v == 0 { 0 } else { 55 + v * 40 };
		(conv(r as u8), conv(g as u8), conv(b as u8))
	} else {
		// grayscale 232..255
		let gray = 8 + (i - 232) * 10;
		(gray as u8, gray as u8, gray as u8)
	}
}

pub fn style_to_css(s: &Style) -> Option<String> {
	let mut props = Vec::new();

	// foreground
	match s.fg {
		Color::Rgb(r, g, b) => {
			props.push(format!("color:{};", rgb_to_hex(r, g, b)));
		}
		Color::Ansi256(idx) => {
			props.push(format!("color:{};", ansi256_to_css_value(idx)));
		}
		Color::None => {}
	}

	// background
	match s.bg {
		Color::Rgb(r, g, b) => {
			props.push(format!("background-color:{};", rgb_to_hex(r, g, b)));
		}
		Color::Ansi256(idx) => {
			props.push(format!("background-color:{};", ansi256_to_css_value(idx)));
		}
		Color::None => {}
	}

	// attributes
	if s.bold {
		props.push("font-weight:700;".into());
	}

	if s.italic {
		props.push("font-style:italic;".into());
	}

	if s.underline {
		props.push("text-decoration:underline;".into());
	}

	if s.reverse {
		match (s.fg, s.bg) {
			(Color::None, Color::None) => {
				props.push("filter:invert(100%);".into());
			}
			(fg, bg) => {
				if let Some(fg_css) = color_to_css_value(fg) {
					props.push(format!("background-color:{};", fg_css));
				}
				if let Some(bg_css) = color_to_css_value(bg) {
					props.push(format!("color:{};", bg_css));
				}
			}
		}
	}

	if props.is_empty() {
		None
	} else {
		Some(props.join(" "))
	}
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
	format!("#{:02x}{:02x}{:02x}", r, g, b)
}

// Convert ANSI colors to CSS values.
// Standard 16 colors use CSS variables; extended colors use RGB.
fn ansi256_to_css_value(idx: u8) -> String {
	const ANSI16: [&str; 16] = [
		"--ansi-black",
		"--ansi-red",
		"--ansi-green",
		"--ansi-yellow",
		"--ansi-blue",
		"--ansi-magenta",
		"--ansi-cyan",
		"--ansi-white",
		"--ansi-bright-black",
		"--ansi-bright-red",
		"--ansi-bright-green",
		"--ansi-bright-yellow",
		"--ansi-bright-blue",
		"--ansi-bright-magenta",
		"--ansi-bright-cyan",
		"--ansi-bright-white",
	];

	if let Some(variable) = ANSI16.get(idx as usize) {
		format!("var({})", variable)
	} else {
		let (r, g, b) = ansi256_to_rgb(idx);
		rgb_to_hex(r, g, b)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rgb_to_hex_conversion() {
		assert_eq!(rgb_to_hex(255, 0, 0), "#ff0000");
		assert_eq!(rgb_to_hex(80, 250, 123), "#50fa7b");
		assert_eq!(rgb_to_hex(0, 0, 0), "#000000");
		assert_eq!(rgb_to_hex(255, 255, 255), "#ffffff");
	}

	#[test]
	fn ansi256_to_hex_conversion() {
		assert_eq!(ansi256_to_css_value(16), "#000000");
		assert_eq!(ansi256_to_css_value(196), "#ff0000");
		assert_eq!(ansi256_to_css_value(231), "#ffffff");
		assert_eq!(ansi256_to_css_value(232), "#080808");
		assert_eq!(ansi256_to_css_value(255), "#eeeeee");
	}

	#[test]
	fn ansi16_uses_css_variables() {
		assert_eq!(ansi256_to_css_value(0), "var(--ansi-black)");
		assert_eq!(ansi256_to_css_value(7), "var(--ansi-white)");
		assert_eq!(ansi256_to_css_value(8), "var(--ansi-bright-black)");
		assert_eq!(ansi256_to_css_value(15), "var(--ansi-bright-white)");
	}
}

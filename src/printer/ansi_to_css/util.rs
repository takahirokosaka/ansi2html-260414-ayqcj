use crate::printer::ansi_to_css::{Color, Style};

pub fn style_to_css(s: &Style) -> Option<String> {
	let mut props = Vec::new();

	// foreground
	match s.fg {
		Color::Rgb(r, g, b) => props.push(format!("color: rgb({},{},{});", r, g, b)),
		Color::Ansi256(idx) => {
			// convert 256-color index to rgb (simple algorithm)
			let rgb = ansi256_to_rgb(idx);
			props.push(format!("color: rgb({},{},{});", rgb.0, rgb.1, rgb.2));
		}
		Color::None => {}
	}

	// background
	match s.bg {
		Color::Rgb(r, g, b) => props.push(format!("background-color: rgb({},{},{});", r, g, b)),
		Color::Ansi256(idx) => {
			let rgb = ansi256_to_rgb(idx);
			props.push(format!(
				"background-color: rgb({},{},{});",
				rgb.0, rgb.1, rgb.2
			));
		}
		Color::None => {}
	}

	// attributes
	if s.bold {
		// bold -> font-weight heavier
		props.push("font-weight: 700;".into());
	}
	if s.italic {
		props.push("font-style: italic;".into());
	}
	if s.underline {
		props.push("text-decoration: underline;".into());
	}
	if s.reverse {
		// reverse: swap fg/bg when both present; otherwise use invert filter
		match (s.fg, s.bg) {
			(Color::None, Color::None) => {
				props.push("filter: invert(100%);".into());
			}
			(fg, bg) => {
				// compute css variables and swap
				if let Some(fg_css) = color_to_css_value(fg) {
					props.push(format!("background-color: {};", fg_css));
				}
				if let Some(bg_css) = color_to_css_value(bg) {
					props.push(format!("color: {};", bg_css));
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

// helper: produce "rgb(r,g,b)" string or hex string
fn color_to_css_value(c: Color) -> Option<String> {
	match c {
		Color::Rgb(r, g, b) => Some(format!("rgb({},{},{})", r, g, b)),
		Color::Ansi256(idx) => {
			let (r, g, b) = ansi256_to_rgb(idx);
			Some(format!("rgb({},{},{})", r, g, b))
		}
		Color::None => None,
	}
}

// convert 256-color palette index to (r,g,b)
fn ansi256_to_rgb(idx: u8) -> (u8, u8, u8) {
	let i = idx as u16;
	if i < 16 {
		//   // standard 16 colors — map to reasonable defaults
		//   let table: [(u8,u8,u8);16] = [
		//       (0,0,0), (128,0,0), (0,128,0), (128,128,0),
		//       (0,0,128), (128,0,128), (0,128,128), (192,192,192),
		//       (128,128,128), (255,0,0), (0,255,0), (255,255,0),
		//       (0,0,255), (255,0,255), (0,255,255), (255,255,255),
		//   ];
		//   return table[i as usize];

		// Dracula palette mapping (normal, bright)
		// TODO: Read from a config file
		let dracula = [
			("#000000", "#21222c"), // black
			("#ff5555", "#ff6e6e"), // red
			("#50fa7b", "#69ff94"), // green
			("#f1fa8c", "#ffffa5"), // yellow
			("#bd93f9", "#caa9ff"), // blue (violet)
			("#ff79c6", "#ff92d0"), // magenta (pink)
			("#8be9fd", "#9bf0ff"), // cyan
			("#bbbbbb", "#ffffff"), // white
		];
		let base = (i % 8) as usize;
		let bright = i >= 8;
		let hex = if bright {
			dracula[base].1
		} else {
			dracula[base].0
		};

		let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
		let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
		let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
		(r, g, b)
	} else if i >= 16 && i <= 231 {
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

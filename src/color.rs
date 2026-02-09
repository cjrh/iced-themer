use iced_core::Color;
use serde::de;
use std::fmt;

/// A newtype around [`Color`] that deserializes from hex strings and named colors.
///
/// Supported formats: `#RGB`, `#RRGGBB`, `#RRGGBBAA`, and named colors
/// (`black`, `white`, `transparent`).
#[derive(Debug, Clone, Copy)]
pub struct HexColor(pub Color);

impl<'de> de::Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_color(&s).map(HexColor).map_err(de::Error::custom)
    }
}

/// Parse a color string into an iced [`Color`].
///
/// Accepts `#RGB`, `#RRGGBB`, `#RRGGBBAA`, and named colors.
pub fn parse_color(s: &str) -> Result<Color, String> {
    match s.to_ascii_lowercase().as_str() {
        "black" => return Ok(Color::BLACK),
        "white" => return Ok(Color::WHITE),
        "transparent" => return Ok(Color::TRANSPARENT),
        _ => {}
    }

    let hex = s.strip_prefix('#').ok_or_else(|| {
        format!("expected '#' prefix or a named color, got \"{s}\"")
    })?;

    match hex.len() {
        3 => {
            let r = parse_hex_digit(hex, 0)?;
            let g = parse_hex_digit(hex, 1)?;
            let b = parse_hex_digit(hex, 2)?;
            Ok(Color::from_rgb8(r << 4 | r, g << 4 | g, b << 4 | b))
        }
        6 => {
            let r = parse_hex_byte(hex, 0)?;
            let g = parse_hex_byte(hex, 2)?;
            let b = parse_hex_byte(hex, 4)?;
            Ok(Color::from_rgb8(r, g, b))
        }
        8 => {
            let r = parse_hex_byte(hex, 0)?;
            let g = parse_hex_byte(hex, 2)?;
            let b = parse_hex_byte(hex, 4)?;
            let a = parse_hex_byte(hex, 6)?;
            Ok(Color::from_rgba8(r, g, b, a as f32 / 255.0))
        }
        n => Err(format!(
            "expected 3, 6, or 8 hex digits after '#', got {n}"
        )),
    }
}

fn parse_hex_digit(hex: &str, pos: usize) -> Result<u8, String> {
    u8::from_str_radix(&hex[pos..pos + 1], 16)
        .map_err(|_| format!("invalid hex digit at position {pos}"))
}

fn parse_hex_byte(hex: &str, pos: usize) -> Result<u8, String> {
    u8::from_str_radix(&hex[pos..pos + 2], 16)
        .map_err(|_| format!("invalid hex byte at position {pos}"))
}

// Implement Display so HexColor can be used in error messages.
impl fmt::Display for HexColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Color { r, g, b, a } = self.0;
        if (a - 1.0).abs() < f32::EPSILON {
            write!(
                f,
                "#{:02X}{:02X}{:02X}",
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8,
            )
        } else {
            write!(
                f,
                "#{:02X}{:02X}{:02X}{:02X}",
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8,
                (a * 255.0) as u8,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: Color, b: Color) -> bool {
        (a.r - b.r).abs() < 0.01
            && (a.g - b.g).abs() < 0.01
            && (a.b - b.b).abs() < 0.01
            && (a.a - b.a).abs() < 0.01
    }

    #[test]
    fn parse_hex_6_digit() {
        let c = parse_color("#FF8000").unwrap();
        assert!(approx_eq(c, Color::from_rgb8(255, 128, 0)));
    }

    #[test]
    fn parse_hex_3_digit() {
        let c = parse_color("#F80").unwrap();
        // #F80 expands to #FF8800
        assert!(approx_eq(c, Color::from_rgb8(0xFF, 0x88, 0x00)));
    }

    #[test]
    fn parse_hex_8_digit() {
        let c = parse_color("#FF800080").unwrap();
        assert!(approx_eq(
            c,
            Color::from_rgba8(255, 128, 0, 128.0 / 255.0)
        ));
    }

    #[test]
    fn parse_named_colors() {
        assert!(approx_eq(parse_color("black").unwrap(), Color::BLACK));
        assert!(approx_eq(parse_color("White").unwrap(), Color::WHITE));
        assert!(approx_eq(
            parse_color("TRANSPARENT").unwrap(),
            Color::TRANSPARENT
        ));
    }

    #[test]
    fn parse_lowercase_hex() {
        let c = parse_color("#ff8000").unwrap();
        assert!(approx_eq(c, Color::from_rgb8(255, 128, 0)));
    }

    #[test]
    fn parse_missing_hash() {
        assert!(parse_color("FF8000").is_err());
    }

    #[test]
    fn parse_wrong_length() {
        assert!(parse_color("#FFFF").is_err());
    }

    #[test]
    fn parse_invalid_hex() {
        assert!(parse_color("#ZZZZZZ").is_err());
    }
}

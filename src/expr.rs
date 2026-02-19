//! Color transformation expression evaluator.
//!
//! Turns strings like `"darken($primary, 20%)"` into resolved hex color strings.
//! All `$variable` arguments must already be resolved to hex strings in `vars`
//! before calling [`evaluate`].

use std::collections::HashMap;

use farver::{Color as _, deg, percent, rgb};
use iced_core::Color;

/// Evaluates a color transformation expression and returns a hex color string.
///
/// `vars` must contain fully-resolved hex strings (no remaining `$refs`).
/// Supports: `darken`, `lighten`, `saturate`, `desaturate`, `tint`, `shade`,
/// `greyscale` / `grayscale`, `spin`, `mix`.
pub(crate) fn evaluate(s: &str, vars: &HashMap<String, String>) -> Result<String, String> {
    let s = s.trim();
    let (fn_name, args_str) = parse_call(s)?;
    let args: Vec<&str> = args_str.split(',').map(str::trim).collect();
    apply(fn_name, &args, vars)
}

// ── Parsing helpers ──────────────────────────────────────────────────────────

fn parse_call(s: &str) -> Result<(&str, &str), String> {
    let (name, rest) = s
        .split_once('(')
        .ok_or_else(|| format!("expected a function call, got `{s}`"))?;
    let args = rest
        .strip_suffix(')')
        .ok_or_else(|| format!("missing closing `)` in `{s}`"))?;
    Ok((name.trim(), args))
}

fn expect_args<'a>(fn_name: &str, args: &'a [&'a str], n: usize) -> Result<&'a [&'a str], String> {
    if args.len() == n {
        Ok(args)
    } else {
        Err(format!(
            "`{fn_name}` expects {n} argument(s), got {}",
            args.len()
        ))
    }
}

// ── Color argument resolution ────────────────────────────────────────────────

/// Resolves a color argument: either a `$variable` reference or a literal color string.
fn resolve_color(s: &str, vars: &HashMap<String, String>) -> Result<Color, String> {
    let literal = if let Some(name) = s.strip_prefix('$') {
        vars.get(name)
            .ok_or_else(|| format!("undefined variable `${name}`"))?
            .as_str()
    } else {
        s
    };
    crate::color::parse_color(literal).map_err(|e| format!("invalid color `{literal}`: {e}"))
}

fn to_farver(c: Color) -> farver::RGB {
    rgb(
        (c.r * 255.0).round() as u8,
        (c.g * 255.0).round() as u8,
        (c.b * 255.0).round() as u8,
    )
}

// ── Parameter parsing ────────────────────────────────────────────────────────

fn parse_percent(s: &str) -> Result<farver::Ratio, String> {
    let digits = s
        .strip_suffix('%')
        .ok_or_else(|| format!("expected a percentage like `20%`, got `{s}`"))?
        .trim();
    let n: u8 = digits
        .parse()
        .map_err(|_| format!("invalid percentage value `{digits}`"))?;
    if n > 100 {
        return Err(format!("percentage must be 0–100, got `{n}`"));
    }
    Ok(percent(n))
}

fn parse_angle(s: &str) -> Result<farver::Angle, String> {
    let digits = s
        .strip_suffix("deg")
        .ok_or_else(|| format!("expected an angle like `180deg`, got `{s}`"))?
        .trim();
    let n: i32 = digits
        .parse()
        .map_err(|_| format!("invalid angle value `{digits}`"))?;
    Ok(deg(n))
}

// ── Dispatch ─────────────────────────────────────────────────────────────────

fn apply(fn_name: &str, args: &[&str], vars: &HashMap<String, String>) -> Result<String, String> {
    match fn_name {
        "darken" => {
            let a = expect_args(fn_name, args, 2)?;
            Ok(to_farver(resolve_color(a[0], vars)?)
                .darken(parse_percent(a[1])?)
                .to_hex())
        }
        "lighten" => {
            let a = expect_args(fn_name, args, 2)?;
            Ok(to_farver(resolve_color(a[0], vars)?)
                .lighten(parse_percent(a[1])?)
                .to_hex())
        }
        "saturate" => {
            let a = expect_args(fn_name, args, 2)?;
            Ok(to_farver(resolve_color(a[0], vars)?)
                .saturate(parse_percent(a[1])?)
                .to_hex())
        }
        "desaturate" => {
            let a = expect_args(fn_name, args, 2)?;
            Ok(to_farver(resolve_color(a[0], vars)?)
                .desaturate(parse_percent(a[1])?)
                .to_hex())
        }
        "tint" => {
            let a = expect_args(fn_name, args, 2)?;
            Ok(to_farver(resolve_color(a[0], vars)?)
                .tint(parse_percent(a[1])?)
                .to_hex())
        }
        "shade" => {
            let a = expect_args(fn_name, args, 2)?;
            Ok(to_farver(resolve_color(a[0], vars)?)
                .shade(parse_percent(a[1])?)
                .to_hex())
        }
        "greyscale" | "grayscale" => {
            let a = expect_args(fn_name, args, 1)?;
            Ok(to_farver(resolve_color(a[0], vars)?).greyscale().to_hex())
        }
        "spin" => {
            let a = expect_args(fn_name, args, 2)?;
            Ok(to_farver(resolve_color(a[0], vars)?)
                .spin(parse_angle(a[1])?)
                .to_hex())
        }
        "mix" => {
            let a = expect_args(fn_name, args, 3)?;
            let c1 = to_farver(resolve_color(a[0], vars)?);
            let c2 = to_farver(resolve_color(a[1], vars)?);
            Ok(c1.mix(c2, parse_percent(a[2])?).to_hex())
        }
        _ => Err(format!("unknown color function `{fn_name}`")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars() -> HashMap<String, String> {
        [
            ("primary".to_string(), "#66C0F4".to_string()),
            ("danger".to_string(), "#F44336".to_string()),
        ]
        .into()
    }

    #[test]
    fn darken_with_variable() {
        let result = evaluate("darken($primary, 20%)", &vars()).unwrap();
        assert!(result.starts_with('#'), "expected hex, got `{result}`");
        assert_eq!(result.len(), 7, "expected #rrggbb, got `{result}`");
    }

    #[test]
    fn lighten_with_literal() {
        let result = evaluate("lighten(#66C0F4, 10%)", &vars()).unwrap();
        assert!(result.starts_with('#'));
    }

    #[test]
    fn greyscale_takes_one_arg() {
        let result = evaluate("greyscale($primary)", &vars()).unwrap();
        assert!(result.starts_with('#'));
    }

    #[test]
    fn grayscale_alias_works() {
        let a = evaluate("greyscale($primary)", &vars()).unwrap();
        let b = evaluate("grayscale($primary)", &vars()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn spin_with_degrees() {
        let result = evaluate("spin($primary, 180deg)", &vars()).unwrap();
        assert!(result.starts_with('#'));
    }

    #[test]
    fn mix_two_variables() {
        let result = evaluate("mix($primary, $danger, 50%)", &vars()).unwrap();
        // mix returns RGBA (#rrggbbaa)
        assert!(result.starts_with('#'));
        assert!(result.len() == 7 || result.len() == 9, "got `{result}`");
    }

    #[test]
    fn unknown_function_returns_error() {
        let err = evaluate("bake($primary, 10%)", &vars()).unwrap_err();
        assert!(err.contains("unknown color function"), "got: {err}");
    }

    #[test]
    fn undefined_variable_returns_error() {
        let err = evaluate("darken($missing, 10%)", &vars()).unwrap_err();
        assert!(err.contains("undefined variable"), "got: {err}");
    }

    #[test]
    fn wrong_arg_count_returns_error() {
        let err = evaluate("darken($primary)", &vars()).unwrap_err();
        assert!(err.contains("expects"), "got: {err}");
    }

    #[test]
    fn percent_out_of_range_returns_error() {
        let err = evaluate("darken($primary, 150%)", &vars()).unwrap_err();
        assert!(err.contains("percentage"), "got: {err}");
    }
}

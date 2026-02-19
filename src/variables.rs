//! Pre-deserialization variable resolution for TOML theme files.
//!
//! Parses the `[variables]` table, resolves variable-to-variable references,
//! then substitutes every `"$name"` string value throughout the TOML tree before
//! serde touches it. This keeps all other parsing logic unchanged.

use std::collections::HashMap;
use toml::Value;

/// Removes `[variables]` from `root` and substitutes all `"$name"` references
/// in the remaining tree. Returns an error string on undefined variables or cycles.
pub(crate) fn resolve(root: &mut Value) -> Result<(), String> {
    let vars = extract(root)?;
    if vars.is_empty() {
        return Ok(());
    }
    let vars = evaluate(vars)?;
    substitute(root, &vars)
}

/// Removes the `[variables]` table from `root` and returns its key→value pairs.
fn extract(root: &mut Value) -> Result<HashMap<String, String>, String> {
    let table = match root.as_table_mut() {
        Some(t) => t,
        None => return Ok(HashMap::new()),
    };

    let vars_value = match table.remove("variables") {
        Some(v) => v,
        None => return Ok(HashMap::new()),
    };

    let vars_table = vars_value
        .as_table()
        .ok_or_else(|| "[variables] must be a TOML table".to_string())?;

    let mut vars = HashMap::new();
    for (key, val) in vars_table {
        match val.as_str() {
            Some(s) => {
                vars.insert(key.clone(), s.to_string());
            }
            None => return Err(format!("variable `{key}` must be a string value")),
        }
    }

    Ok(vars)
}

/// Resolves variable-to-variable references iteratively. Detects cycles and
/// undefined references, returning a descriptive error for each.
fn evaluate(mut vars: HashMap<String, String>) -> Result<HashMap<String, String>, String> {
    // One pass per variable is sufficient for any non-cyclic chain.
    for _ in 0..=vars.len() {
        let snapshot = vars.clone();
        let mut changed = false;

        for (key, val) in vars.iter_mut() {
            if let Some(name) = val.strip_prefix('$') {
                match snapshot.get(name) {
                    Some(resolved) if resolved != val => {
                        *val = resolved.clone();
                        changed = true;
                    }
                    Some(_) => {} // value unchanged; will be caught by post-loop check
                    None => {
                        return Err(format!(
                            "undefined variable `${name}` (referenced from `${key}`)"
                        ));
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    // Any remaining `$ref` values indicate a cycle.
    let cyclic: Vec<String> = vars
        .iter()
        .filter(|(_, v)| v.starts_with('$'))
        .map(|(k, _)| format!("`${k}`"))
        .collect();

    if !cyclic.is_empty() {
        return Err(format!(
            "cyclic variable references: {}",
            cyclic.join(", ")
        ));
    }

    Ok(vars)
}

/// Walks `value` recursively, replacing whole-string `"$name"` values with the
/// resolved color from `vars`. Returns an error for any unresolved `$` reference.
fn substitute(value: &mut Value, vars: &HashMap<String, String>) -> Result<(), String> {
    match value {
        Value::String(s) => {
            if let Some(name) = s.strip_prefix('$') {
                match vars.get(name) {
                    Some(resolved) => *s = resolved.clone(),
                    None => return Err(format!("undefined variable `${name}`")),
                }
            }
        }
        Value::Array(arr) => {
            for item in arr {
                substitute(item, vars)?;
            }
        }
        Value::Table(table) => {
            for (_, val) in table.iter_mut() {
                substitute(val, vars)?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Value {
        toml::from_str(s).unwrap()
    }

    #[test]
    fn no_variables_section_is_a_noop() {
        let mut v = parse(
            r##"
[palette]
primary = "#FF0000"
"##,
        );
        resolve(&mut v).unwrap();
        assert_eq!(v["palette"]["primary"].as_str(), Some("#FF0000"));
    }

    #[test]
    fn basic_substitution() {
        let mut v = parse(
            r##"
[variables]
primary = "#66C0F4"

[button]
background = "$primary"
"##,
        );
        resolve(&mut v).unwrap();
        assert!(v.as_table().unwrap().get("variables").is_none());
        assert_eq!(v["button"]["background"].as_str(), Some("#66C0F4"));
    }

    #[test]
    fn variable_to_variable_reference() {
        let mut v = parse(
            r##"
[variables]
primary = "#66C0F4"
muted   = "$primary"

[button]
background = "$muted"
"##,
        );
        resolve(&mut v).unwrap();
        assert_eq!(v["button"]["background"].as_str(), Some("#66C0F4"));
    }

    #[test]
    fn substitution_inside_gradient_stops() {
        let mut v = parse(
            r##"
[variables]
start = "#1B2838"
end   = "#2A3F5F"

[progress-bar.background]
angle = 0
stops = [
  { offset = 0.0, color = "$start" },
  { offset = 1.0, color = "$end"   },
]
"##,
        );
        resolve(&mut v).unwrap();
        let stops = v["progress-bar"]["background"]["stops"].as_array().unwrap();
        assert_eq!(stops[0]["color"].as_str(), Some("#1B2838"));
        assert_eq!(stops[1]["color"].as_str(), Some("#2A3F5F"));
    }

    #[test]
    fn undefined_variable_returns_error() {
        let mut v = parse(
            r#"
[variables]
primary = "$missing"

[button]
background = "$primary"
"#,
        );
        let err = resolve(&mut v).unwrap_err();
        assert!(err.contains("undefined variable"), "got: {err}");
    }

    #[test]
    fn undefined_variable_in_toml_body_returns_error() {
        let mut v = parse(
            r##"
[variables]
primary = "#FF0000"

[button]
background = "$undefined"
"##,
        );
        let err = resolve(&mut v).unwrap_err();
        assert!(err.contains("undefined variable"), "got: {err}");
    }

    #[test]
    fn cycle_detection() {
        let mut v = parse(
            r#"
[variables]
a = "$b"
b = "$a"

[palette]
primary = "$a"
"#,
        );
        let err = resolve(&mut v).unwrap_err();
        assert!(err.contains("cyclic"), "got: {err}");
    }

    #[test]
    fn non_dollar_strings_are_unchanged() {
        let mut v = parse(
            r##"
[variables]
primary = "#FF0000"

[font]
family = "Arial"
"##,
        );
        resolve(&mut v).unwrap();
        assert_eq!(v["font"]["family"].as_str(), Some("Arial"));
    }
}

use std::io;

/// Errors that can occur when loading or parsing a theme.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to read the theme file.
    #[error("failed to read theme file: {0}")]
    Io(#[from] io::Error),

    /// The TOML content could not be parsed.
    #[error("failed to parse theme: {0}")]
    Parse(#[from] toml::de::Error),

    /// A color value was invalid.
    #[error("invalid color for `{field}`: \"{value}\" ({reason})")]
    InvalidColor {
        field: String,
        value: String,
        reason: String,
    },
}

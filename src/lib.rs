//! Parse TOML theme files into iced's native [`Theme`] type.
//!
//! `iced-themer` lets users define themes declaratively in TOML and load them
//! at runtime, avoiding recompilation when colors or fonts change.
//!
//! # Quick start
//!
//! ```no_run
//! use iced_themer::ThemeConfig;
//!
//! let config = ThemeConfig::from_file("theme.toml").unwrap();
//! let theme = config.theme();   // cheap Arc clone
//! let font = config.font();     // Option<Font>
//! ```

mod color;
mod config;
mod error;
pub mod style;

pub use error::Error;

use iced_core::font::Font;
use iced_core::theme::Theme;
use std::path::Path;
use std::str::FromStr;

use style::*;

/// A parsed theme configuration ready for use with iced.
///
/// Constructed from a TOML string or file, `ThemeConfig` eagerly validates and
/// converts the theme data into iced types. Accessor methods are cheap: `theme()`
/// clones an `Arc`, and `font()` copies a `Copy` type. Widget style accessors
/// return `Option<&Style>` — `None` when the TOML omits that widget's section.
pub struct ThemeConfig {
    pub(crate) name: String,
    pub(crate) theme: Theme,
    pub(crate) font: Option<Font>,
    pub(crate) button: Option<ButtonStyle>,
    pub(crate) container: Option<ContainerStyle>,
    pub(crate) text_input: Option<TextInputStyle>,
    pub(crate) checkbox: Option<CheckboxStyle>,
    pub(crate) toggler: Option<TogglerStyle>,
    pub(crate) slider: Option<SliderStyle>,
    pub(crate) progress_bar: Option<ProgressBarStyle>,
    pub(crate) radio: Option<RadioStyle>,
}

impl ThemeConfig {
    /// Read and parse a TOML theme file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let contents = std::fs::read_to_string(path)?;
        contents.parse()
    }

    /// The theme name. Defaults to `"Custom"` if not specified in the TOML.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns an iced [`Theme`]. This is a cheap `Arc` clone.
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    /// Returns the configured [`Font`], if one was specified in the TOML.
    pub fn font(&self) -> Option<Font> {
        self.font
    }

    pub fn button(&self) -> Option<&ButtonStyle> {
        self.button.as_ref()
    }

    pub fn container(&self) -> Option<&ContainerStyle> {
        self.container.as_ref()
    }

    pub fn text_input(&self) -> Option<&TextInputStyle> {
        self.text_input.as_ref()
    }

    pub fn checkbox(&self) -> Option<&CheckboxStyle> {
        self.checkbox.as_ref()
    }

    pub fn toggler(&self) -> Option<&TogglerStyle> {
        self.toggler.as_ref()
    }

    pub fn slider(&self) -> Option<&SliderStyle> {
        self.slider.as_ref()
    }

    pub fn progress_bar(&self) -> Option<&ProgressBarStyle> {
        self.progress_bar.as_ref()
    }

    pub fn radio(&self) -> Option<&RadioStyle> {
        self.radio.as_ref()
    }
}

impl FromStr for ThemeConfig {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw: config::ThemeRaw = toml::from_str(s)?;
        raw.try_into()
    }
}

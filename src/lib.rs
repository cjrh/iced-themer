//! Parse TOML theme files into iced's native [`Theme`] type at runtime.
//!
//! `iced-themer` reads a `.toml` file and turns it into types that iced
//! understands, so you can change colors, fonts, and widget styles without
//! recompiling your program.
//!
//! # Two levels of theming
//!
//! A theme file has two kinds of sections, and they work differently:
//!
//! ## 1. Palette and font — automatic, app-wide
//!
//! The `[palette]` and `[font]` sections set the overall look of your app.
//! You apply them once when creating the application and every widget picks
//! them up automatically — no per-widget code needed.
//!
//! ```toml
//! name = "Ocean Breeze"
//!
//! [palette]
//! background = "#1B2838"
//! text       = "#C7D5E0"
//! primary    = "#66C0F4"
//! success    = "#4CAF50"
//! warning    = "#FFC107"
//! danger     = "#F44336"
//!
//! [font]
//! family = "Arial"
//! weight = "normal"
//! ```
//!
//! ```no_run
//! use iced::Theme;
//! use iced_themer::ThemeConfig;
//!
//! let config = ThemeConfig::from_file("theme.toml").unwrap();
//! let theme = config.theme();
//! let font  = config.font();
//!
//! let app = iced::application(|| MyApp, MyApp::update, MyApp::view)
//!     .theme(move |_: &MyApp| -> Theme { theme.clone() });
//!
//! // If a font was specified, set it as the default:
//! # struct MyApp; impl MyApp { fn update(&mut self, _: ()) {} fn view(&self) -> iced::Element<'_, ()> { todo!() } }
//! match font {
//!     Some(f) => app.default_font(f).run(),
//!     None    => app.run(),
//! }
//! # ;
//! ```
//!
//! With just a `[palette]`, buttons will use the primary color, text will use
//! the text color, backgrounds will use the background color, and so on. This
//! is iced's built-in theming at work — the palette flows through every widget
//! without you having to touch each one.
//!
//! ## 2. Widget styles — opt-in, per-widget
//!
//! Sometimes the palette isn't enough. Maybe you want a button with a specific
//! hex background that doesn't match any palette slot, or you want the border
//! to change color when hovered. That's what the widget sections are for:
//!
//! ```toml
//! [button]
//! background   = "#66C0F4"
//! text-color   = "#FFFFFF"
//! border-radius = 4.0
//!
//! [button.hovered]
//! background = "#77D0FF"
//!
//! [button.pressed]
//! background = "#5590C0"
//! ```
//!
//! These sections are **optional** — if you leave them out, the widget just
//! uses whatever the palette gives it. Status sub-tables (`hovered`, `pressed`,
//! `disabled`, etc.) inherit every field from the base and only override what
//! they specify.
//!
//! Unlike the palette, widget styles must be applied to each widget explicitly.
//! Every style type has a [`style_fn()`] method that returns a closure you can
//! pass straight to the widget's `.style()` builder:
//!
//! ```no_run
//! # use iced::widget::button;
//! # use iced_themer::ThemeConfig;
//! # let config = ThemeConfig::from_file("theme.toml").unwrap();
//! let mut btn = button("Click me").on_press(());
//!
//! if let Some(s) = config.button() {
//!     btn = btn.style(s.style_fn());
//! }
//! ```
//!
//! The `if let Some` handles the case where the TOML file doesn't include a
//! `[button]` section — the widget will just use the palette defaults.
//!
//! [`style_fn()`]: style::ButtonStyle::style_fn
//!
//! # Supported widget sections
//!
//! | TOML section      | Style type                          |
//! |-------------------|-------------------------------------|
//! | `[button]`        | [`ButtonStyle`](style::ButtonStyle) |
//! | `[checkbox]`      | [`CheckboxStyle`](style::CheckboxStyle) |
//! | `[container]`     | [`ContainerStyle`](style::ContainerStyle) |
//! | `[progress-bar]`  | [`ProgressBarStyle`](style::ProgressBarStyle) |
//! | `[radio]`         | [`RadioStyle`](style::RadioStyle) |
//! | `[slider]`        | [`SliderStyle`](style::SliderStyle) |
//! | `[text-input]`    | [`TextInputStyle`](style::TextInputStyle) |
//! | `[toggler]`       | [`TogglerStyle`](style::TogglerStyle) |

mod color;
mod config;
mod error;
pub mod style;
mod variables;

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
        let mut value: toml::Value = toml::from_str(s)?;
        variables::resolve(&mut value).map_err(|reason| Error::InvalidColor {
            field: "variables".to_string(),
            value: String::new(),
            reason,
        })?;
        let raw: config::ThemeRaw = serde::Deserialize::deserialize(value)?;
        raw.try_into()
    }
}

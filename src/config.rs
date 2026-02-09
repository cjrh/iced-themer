use iced_core::font::{self, Font};
use iced_core::theme::{Palette, Theme};
use serde::Deserialize;

use crate::color::HexColor;
use crate::error::Error;
use crate::style::{
    ButtonSection, CheckboxSection, ContainerSection, ProgressBarSection,
    RadioSection, SliderSection, TextInputSection, TogglerSection,
};
use crate::ThemeConfig;

/// Raw top-level TOML structure, before conversion to iced types.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct ThemeRaw {
    pub name: Option<String>,
    pub palette: PaletteRaw,
    pub font: Option<FontRaw>,
    pub button: Option<ButtonSection>,
    pub container: Option<ContainerSection>,
    pub text_input: Option<TextInputSection>,
    pub checkbox: Option<CheckboxSection>,
    pub toggler: Option<TogglerSection>,
    pub slider: Option<SliderSection>,
    pub progress_bar: Option<ProgressBarSection>,
    pub radio: Option<RadioSection>,
}

/// The 6 semantic colors that make up an iced palette.
#[derive(Deserialize)]
pub(crate) struct PaletteRaw {
    pub background: HexColor,
    pub text: HexColor,
    pub primary: HexColor,
    pub success: HexColor,
    pub warning: HexColor,
    pub danger: HexColor,
}

/// Optional font configuration. All fields default to iced's defaults when absent.
#[derive(Deserialize)]
pub(crate) struct FontRaw {
    pub family: Option<String>,
    pub weight: Option<FontWeight>,
    pub style: Option<FontStyle>,
    pub stretch: Option<FontStretch>,
}

// Mirror enums for serde -- iced_core's enums don't derive Deserialize.

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

impl From<FontWeight> for font::Weight {
    fn from(w: FontWeight) -> Self {
        match w {
            FontWeight::Thin => font::Weight::Thin,
            FontWeight::ExtraLight => font::Weight::ExtraLight,
            FontWeight::Light => font::Weight::Light,
            FontWeight::Normal => font::Weight::Normal,
            FontWeight::Medium => font::Weight::Medium,
            FontWeight::Semibold => font::Weight::Semibold,
            FontWeight::Bold => font::Weight::Bold,
            FontWeight::ExtraBold => font::Weight::ExtraBold,
            FontWeight::Black => font::Weight::Black,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl From<FontStyle> for font::Style {
    fn from(s: FontStyle) -> Self {
        match s {
            FontStyle::Normal => font::Style::Normal,
            FontStyle::Italic => font::Style::Italic,
            FontStyle::Oblique => font::Style::Oblique,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum FontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl From<FontStretch> for font::Stretch {
    fn from(s: FontStretch) -> Self {
        match s {
            FontStretch::UltraCondensed => font::Stretch::UltraCondensed,
            FontStretch::ExtraCondensed => font::Stretch::ExtraCondensed,
            FontStretch::Condensed => font::Stretch::Condensed,
            FontStretch::SemiCondensed => font::Stretch::SemiCondensed,
            FontStretch::Normal => font::Stretch::Normal,
            FontStretch::SemiExpanded => font::Stretch::SemiExpanded,
            FontStretch::Expanded => font::Stretch::Expanded,
            FontStretch::ExtraExpanded => font::Stretch::ExtraExpanded,
            FontStretch::UltraExpanded => font::Stretch::UltraExpanded,
        }
    }
}

impl TryFrom<ThemeRaw> for ThemeConfig {
    type Error = Error;

    fn try_from(raw: ThemeRaw) -> Result<Self, Self::Error> {
        let name = raw.name.unwrap_or_else(|| "Custom".to_string());

        let palette = Palette {
            background: raw.palette.background.0,
            text: raw.palette.text.0,
            primary: raw.palette.primary.0,
            success: raw.palette.success.0,
            warning: raw.palette.warning.0,
            danger: raw.palette.danger.0,
        };

        let theme = Theme::custom(name.clone(), palette);

        let font = raw.font.map(build_font);

        let button = raw.button.map(|s| s.resolve());
        let container = raw.container.map(|s| s.resolve());
        let text_input = raw.text_input.map(|s| s.resolve());
        let checkbox = raw.checkbox.map(|s| s.resolve());
        let toggler = raw.toggler.map(|s| s.resolve());
        let slider = raw.slider.map(|s| s.resolve());
        let progress_bar = raw.progress_bar.map(|s| s.resolve());
        let radio = raw.radio.map(|s| s.resolve());

        Ok(ThemeConfig {
            name,
            theme,
            font,
            button,
            container,
            text_input,
            checkbox,
            toggler,
            slider,
            progress_bar,
            radio,
        })
    }
}

fn build_font(raw: FontRaw) -> Font {
    let family = match raw.family.as_deref() {
        None | Some("sans-serif") => font::Family::SansSerif,
        Some("serif") => font::Family::Serif,
        Some("monospace") => font::Family::Monospace,
        Some("cursive") => font::Family::Cursive,
        Some("fantasy") => font::Family::Fantasy,
        Some(custom) => {
            let leaked: &'static str = Box::leak(custom.to_string().into_boxed_str());
            font::Family::Name(leaked)
        }
    };

    Font {
        family,
        weight: raw.weight.map(Into::into).unwrap_or(font::Weight::Normal),
        stretch: raw.stretch.map(Into::into).unwrap_or(font::Stretch::Normal),
        style: raw.style.map(Into::into).unwrap_or(font::Style::Normal),
    }
}

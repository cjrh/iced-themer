use iced_core::{Background, Border, Color};
use serde::Deserialize;

use crate::color::HexColor;
use super::{RadiusRaw, impl_merge, resolve_border};

// -- Layer 1: Serde raw types --

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct TextInputFieldsRaw {
    background:        Option<HexColor>,
    border_width:      Option<f32>,
    border_color:      Option<HexColor>,
    border_radius:     Option<RadiusRaw>,
    icon_color:        Option<HexColor>,
    placeholder_color: Option<HexColor>,
    value_color:       Option<HexColor>,
    selection_color:   Option<HexColor>,
}

impl_merge!(TextInputFieldsRaw {
    background, border_width, border_color, border_radius,
    icon_color, placeholder_color, value_color, selection_color,
});

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct TextInputSection {
    #[serde(flatten)]
    base: TextInputFieldsRaw,
    focused:  Option<TextInputFieldsRaw>,
    disabled: Option<TextInputFieldsRaw>,
}

// -- Layer 2: Resolution --

impl TextInputSection {
    pub fn resolve(self) -> TextInputStyle {
        let active = into_appearance(self.base);
        let focused = resolve_status(self.base, self.focused.as_ref());
        let disabled = resolve_status(self.base, self.disabled.as_ref());

        TextInputStyle { active, focused, disabled }
    }
}

fn resolve_status(base: TextInputFieldsRaw, status: Option<&TextInputFieldsRaw>) -> TextInputAppearance {
    match status {
        Some(over) => into_appearance(base.merge(over)),
        None => into_appearance(base),
    }
}

fn into_appearance(f: TextInputFieldsRaw) -> TextInputAppearance {
    TextInputAppearance {
        background: Background::Color(f.background.map(|c| c.0).unwrap_or(Color::TRANSPARENT)),
        border: resolve_border(f.border_width, f.border_color, f.border_radius),
        icon_color: f.icon_color.map(|c| c.0).unwrap_or(Color::BLACK),
        placeholder_color: f.placeholder_color.map(|c| c.0).unwrap_or(Color::from_rgba8(0x80, 0x80, 0x80, 1.0)),
        value_color: f.value_color.map(|c| c.0).unwrap_or(Color::BLACK),
        selection_color: f.selection_color.map(|c| c.0).unwrap_or(Color::from_rgba8(0x33, 0x99, 0xFF, 0.3)),
    }
}

// -- Layer 3: Public types --

/// Pre-resolved text input style with an appearance for each status variant.
#[derive(Debug, Clone)]
pub struct TextInputStyle {
    active:   TextInputAppearance,
    focused:  TextInputAppearance,
    disabled: TextInputAppearance,
}

impl TextInputStyle {
    pub fn active(&self) -> &TextInputAppearance {
        &self.active
    }

    pub fn focused(&self) -> &TextInputAppearance {
        &self.focused
    }

    pub fn disabled(&self) -> &TextInputAppearance {
        &self.disabled
    }
}

/// Visual properties for a text input. Fields mirror `iced_widget::text_input::Style`.
#[derive(Debug, Clone, Copy)]
pub struct TextInputAppearance {
    pub background: Background,
    pub border: Border,
    pub icon_color: Color,
    pub placeholder_color: Color,
    pub value_color: Color,
    pub selection_color: Color,
}

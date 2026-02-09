use iced_core::{Background, Color, Theme};
use iced_widget::text_input;
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
        let active = into_native(self.base);
        let focused = resolve_status(self.base, self.focused.as_ref());
        let disabled = resolve_status(self.base, self.disabled.as_ref());

        TextInputStyle { active, focused, disabled }
    }
}

fn resolve_status(base: TextInputFieldsRaw, status: Option<&TextInputFieldsRaw>) -> text_input::Style {
    match status {
        Some(over) => into_native(base.merge(over)),
        None => into_native(base),
    }
}

fn into_native(f: TextInputFieldsRaw) -> text_input::Style {
    text_input::Style {
        background: Background::Color(f.background.map(|c| c.0).unwrap_or(Color::TRANSPARENT)),
        border: resolve_border(f.border_width, f.border_color, f.border_radius),
        icon: f.icon_color.map(|c| c.0).unwrap_or(Color::BLACK),
        placeholder: f.placeholder_color.map(|c| c.0).unwrap_or(Color::from_rgba8(0x80, 0x80, 0x80, 1.0)),
        value: f.value_color.map(|c| c.0).unwrap_or(Color::BLACK),
        selection: f.selection_color.map(|c| c.0).unwrap_or(Color::from_rgba8(0x33, 0x99, 0xFF, 0.3)),
    }
}

// -- Layer 3: Public types --

/// Pre-resolved text input style with a native `iced_widget` style for each status variant.
#[derive(Debug, Clone, Copy)]
pub struct TextInputStyle {
    active:   text_input::Style,
    focused:  text_input::Style,
    disabled: text_input::Style,
}

impl TextInputStyle {
    /// Returns a closure suitable for passing to `.style()` on a text input widget.
    ///
    /// The `Hovered` status maps to the active style, and `Focused { is_hovered: _ }`
    /// maps to the focused style, matching iced's status semantics.
    pub fn style_fn(&self) -> impl Fn(&Theme, text_input::Status) -> text_input::Style + Copy {
        let s = *self;
        move |_theme, status| match status {
            text_input::Status::Active  => s.active,
            text_input::Status::Hovered => s.active,
            text_input::Status::Focused { .. } => s.focused,
            text_input::Status::Disabled => s.disabled,
        }
    }
}

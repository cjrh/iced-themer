use iced_core::{Background, Color, Theme};
use iced_widget::button;
use serde::Deserialize;

use crate::color::HexColor;
use super::{RadiusRaw, impl_merge, resolve_border, resolve_shadow};

// -- Layer 1: Serde raw types --

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct ButtonFieldsRaw {
    background:         Option<HexColor>,
    text_color:         Option<HexColor>,
    border_width:       Option<f32>,
    border_color:       Option<HexColor>,
    border_radius:      Option<RadiusRaw>,
    shadow_color:       Option<HexColor>,
    shadow_offset_x:    Option<f32>,
    shadow_offset_y:    Option<f32>,
    shadow_blur_radius: Option<f32>,
}

impl_merge!(ButtonFieldsRaw {
    background, text_color,
    border_width, border_color, border_radius,
    shadow_color, shadow_offset_x, shadow_offset_y, shadow_blur_radius,
});

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct ButtonSection {
    #[serde(flatten)]
    base: ButtonFieldsRaw,
    hovered:  Option<ButtonFieldsRaw>,
    pressed:  Option<ButtonFieldsRaw>,
    disabled: Option<ButtonFieldsRaw>,
}

// -- Layer 2: Resolution --

impl ButtonSection {
    pub fn resolve(self) -> ButtonStyle {
        let active = into_native(self.base);
        let hovered = resolve_status(self.base, self.hovered.as_ref());
        let pressed = resolve_status(self.base, self.pressed.as_ref());
        let disabled = resolve_status(self.base, self.disabled.as_ref());

        ButtonStyle { active, hovered, pressed, disabled }
    }
}

fn resolve_status(base: ButtonFieldsRaw, status: Option<&ButtonFieldsRaw>) -> button::Style {
    match status {
        Some(over) => into_native(base.merge(over)),
        None => into_native(base),
    }
}

fn into_native(f: ButtonFieldsRaw) -> button::Style {
    button::Style {
        background: f.background.map(|c| Background::Color(c.0)),
        text_color: f.text_color.map(|c| c.0).unwrap_or(Color::BLACK),
        border: resolve_border(f.border_width, f.border_color, f.border_radius),
        shadow: resolve_shadow(f.shadow_color, f.shadow_offset_x, f.shadow_offset_y, f.shadow_blur_radius),
        snap: false,
    }
}

// -- Layer 3: Public types --

/// Pre-resolved button style with a native `iced_widget` style for each status variant.
#[derive(Debug, Clone, Copy)]
pub struct ButtonStyle {
    active:   button::Style,
    hovered:  button::Style,
    pressed:  button::Style,
    disabled: button::Style,
}

impl ButtonStyle {
    /// Returns a closure suitable for passing to `.style()` on a button widget.
    pub fn style_fn(&self) -> impl Fn(&Theme, button::Status) -> button::Style + Copy {
        let s = *self;
        move |_theme, status| match status {
            button::Status::Active  => s.active,
            button::Status::Hovered => s.hovered,
            button::Status::Pressed => s.pressed,
            button::Status::Disabled => s.disabled,
        }
    }
}

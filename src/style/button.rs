use iced_core::{Background, Border, Color, Shadow};
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
        let active = into_appearance(self.base);
        let hovered = resolve_status(self.base, self.hovered.as_ref());
        let pressed = resolve_status(self.base, self.pressed.as_ref());
        let disabled = resolve_status(self.base, self.disabled.as_ref());

        ButtonStyle { active, hovered, pressed, disabled }
    }
}

fn resolve_status(base: ButtonFieldsRaw, status: Option<&ButtonFieldsRaw>) -> ButtonAppearance {
    match status {
        Some(over) => into_appearance(base.merge(over)),
        None => into_appearance(base),
    }
}

fn into_appearance(f: ButtonFieldsRaw) -> ButtonAppearance {
    ButtonAppearance {
        background: f.background.map(|c| Background::Color(c.0)),
        text_color: f.text_color.map(|c| c.0).unwrap_or(Color::BLACK),
        border: resolve_border(f.border_width, f.border_color, f.border_radius),
        shadow: resolve_shadow(f.shadow_color, f.shadow_offset_x, f.shadow_offset_y, f.shadow_blur_radius),
    }
}

// -- Layer 3: Public types --

/// Pre-resolved button style with an appearance for each status variant.
#[derive(Debug, Clone)]
pub struct ButtonStyle {
    active:   ButtonAppearance,
    hovered:  ButtonAppearance,
    pressed:  ButtonAppearance,
    disabled: ButtonAppearance,
}

impl ButtonStyle {
    pub fn active(&self) -> &ButtonAppearance {
        &self.active
    }

    pub fn hovered(&self) -> &ButtonAppearance {
        &self.hovered
    }

    pub fn pressed(&self) -> &ButtonAppearance {
        &self.pressed
    }

    pub fn disabled(&self) -> &ButtonAppearance {
        &self.disabled
    }
}

/// Visual properties for a button. Fields mirror `iced_widget::button::Style`.
#[derive(Debug, Clone, Copy)]
pub struct ButtonAppearance {
    pub background: Option<Background>,
    pub text_color: Color,
    pub border: Border,
    pub shadow: Shadow,
}

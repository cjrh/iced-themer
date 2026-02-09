use iced_core::Color;
use serde::Deserialize;

use crate::color::HexColor;
use super::impl_merge;

// -- Layer 1: Serde raw types --

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct TogglerFieldsRaw {
    background:              Option<HexColor>,
    foreground:              Option<HexColor>,
    background_border_width: Option<f32>,
    background_border_color: Option<HexColor>,
    foreground_border_width: Option<f32>,
    foreground_border_color: Option<HexColor>,
    border_radius:           Option<f32>,
    text_color:              Option<HexColor>,
}

impl_merge!(TogglerFieldsRaw {
    background, foreground,
    background_border_width, background_border_color,
    foreground_border_width, foreground_border_color,
    border_radius, text_color,
});

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct TogglerSection {
    #[serde(flatten)]
    base: TogglerFieldsRaw,
    toggled: Option<TogglerFieldsRaw>,
    hovered: Option<TogglerFieldsRaw>,
    disabled: Option<TogglerFieldsRaw>,
    hovered_toggled: Option<TogglerFieldsRaw>,
    disabled_toggled: Option<TogglerFieldsRaw>,
}

// -- Layer 2: Resolution --

fn cascade(
    base: TogglerFieldsRaw,
    state: Option<&TogglerFieldsRaw>,
    status: Option<&TogglerFieldsRaw>,
    combined: Option<&TogglerFieldsRaw>,
) -> TogglerAppearance {
    let mut resolved = base;
    if let Some(s) = state {
        resolved = resolved.merge(s);
    }
    if let Some(s) = status {
        resolved = resolved.merge(s);
    }
    if let Some(c) = combined {
        resolved = resolved.merge(c);
    }
    into_appearance(resolved)
}

impl TogglerSection {
    pub fn resolve(self) -> TogglerStyle {
        let active_untoggled = into_appearance(self.base);
        let active_toggled = cascade(self.base, self.toggled.as_ref(), None, None);
        let hovered_untoggled = cascade(self.base, None, self.hovered.as_ref(), None);
        let hovered_toggled = cascade(self.base, self.toggled.as_ref(), self.hovered.as_ref(), self.hovered_toggled.as_ref());
        let disabled_untoggled = cascade(self.base, None, self.disabled.as_ref(), None);
        let disabled_toggled = cascade(self.base, self.toggled.as_ref(), self.disabled.as_ref(), self.disabled_toggled.as_ref());

        TogglerStyle {
            active_untoggled,
            active_toggled,
            hovered_untoggled,
            hovered_toggled,
            disabled_untoggled,
            disabled_toggled,
        }
    }
}

fn into_appearance(f: TogglerFieldsRaw) -> TogglerAppearance {
    TogglerAppearance {
        background: f.background.map(|c| c.0).unwrap_or(Color::TRANSPARENT),
        foreground: f.foreground.map(|c| c.0).unwrap_or(Color::BLACK),
        background_border_width: f.background_border_width.unwrap_or(0.0),
        background_border_color: f.background_border_color.map(|c| c.0).unwrap_or(Color::TRANSPARENT),
        foreground_border_width: f.foreground_border_width.unwrap_or(0.0),
        foreground_border_color: f.foreground_border_color.map(|c| c.0).unwrap_or(Color::TRANSPARENT),
        border_radius: f.border_radius,
        text_color: f.text_color.map(|c| c.0),
    }
}

// -- Layer 3: Public types --

/// Pre-resolved toggler style with 6 variants (3 statuses × 2 states).
#[derive(Debug, Clone)]
pub struct TogglerStyle {
    active_untoggled:   TogglerAppearance,
    active_toggled:     TogglerAppearance,
    hovered_untoggled:  TogglerAppearance,
    hovered_toggled:    TogglerAppearance,
    disabled_untoggled: TogglerAppearance,
    disabled_toggled:   TogglerAppearance,
}

impl TogglerStyle {
    pub fn active(&self, is_toggled: bool) -> &TogglerAppearance {
        if is_toggled { &self.active_toggled } else { &self.active_untoggled }
    }

    pub fn hovered(&self, is_toggled: bool) -> &TogglerAppearance {
        if is_toggled { &self.hovered_toggled } else { &self.hovered_untoggled }
    }

    pub fn disabled(&self, is_toggled: bool) -> &TogglerAppearance {
        if is_toggled { &self.disabled_toggled } else { &self.disabled_untoggled }
    }
}

/// Visual properties for a toggler. Fields mirror `iced_widget::toggler::Style`.
#[derive(Debug, Clone, Copy)]
pub struct TogglerAppearance {
    pub background: Color,
    pub foreground: Color,
    pub background_border_width: f32,
    pub background_border_color: Color,
    pub foreground_border_width: f32,
    pub foreground_border_color: Color,
    /// If `None`, inherits iced's default.
    pub border_radius: Option<f32>,
    pub text_color: Option<Color>,
}

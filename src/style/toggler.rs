use iced_core::{Background, Color, Theme};
use iced_widget::toggler;
use serde::Deserialize;

use crate::color::HexColor;
use super::{BackgroundRaw, impl_merge};

// -- Layer 1: Serde raw types --

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct TogglerFieldsRaw {
    background:              Option<BackgroundRaw>,
    foreground:              Option<BackgroundRaw>,
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

/// Cascade: base -> state -> status -> combined
fn cascade(
    base: TogglerFieldsRaw,
    state: Option<&TogglerFieldsRaw>,
    status: Option<&TogglerFieldsRaw>,
    combined: Option<&TogglerFieldsRaw>,
) -> toggler::Style {
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
    into_native(resolved)
}

impl TogglerSection {
    pub fn resolve(self) -> TogglerStyle {
        let active_untoggled = into_native(self.base);
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

fn into_native(f: TogglerFieldsRaw) -> toggler::Style {
    toggler::Style {
        background: f.background.map(BackgroundRaw::into_background).unwrap_or(Background::Color(Color::TRANSPARENT)),
        foreground: f.foreground.map(BackgroundRaw::into_background).unwrap_or(Background::Color(Color::BLACK)),
        background_border_width: f.background_border_width.unwrap_or(0.0),
        background_border_color: f.background_border_color.map(|c| c.0).unwrap_or(Color::TRANSPARENT),
        foreground_border_width: f.foreground_border_width.unwrap_or(0.0),
        foreground_border_color: f.foreground_border_color.map(|c| c.0).unwrap_or(Color::TRANSPARENT),
        border_radius: f.border_radius.map(|r| r.into()),
        text_color: f.text_color.map(|c| c.0),
        padding_ratio: 0.36,
    }
}

// -- Layer 3: Public types --

/// Pre-resolved toggler style with 6 variants (3 statuses x 2 states).
#[derive(Debug, Clone, Copy)]
pub struct TogglerStyle {
    active_untoggled:   toggler::Style,
    active_toggled:     toggler::Style,
    hovered_untoggled:  toggler::Style,
    hovered_toggled:    toggler::Style,
    disabled_untoggled: toggler::Style,
    disabled_toggled:   toggler::Style,
}

impl TogglerStyle {
    /// Returns a closure suitable for passing to `.style()` on a toggler widget.
    pub fn style_fn(&self) -> impl Fn(&Theme, toggler::Status) -> toggler::Style + Copy {
        let s = *self;
        move |_theme, status| match status {
            toggler::Status::Active { is_toggled } => {
                if is_toggled { s.active_toggled } else { s.active_untoggled }
            }
            toggler::Status::Hovered { is_toggled } => {
                if is_toggled { s.hovered_toggled } else { s.hovered_untoggled }
            }
            toggler::Status::Disabled { is_toggled } => {
                if is_toggled { s.disabled_toggled } else { s.disabled_untoggled }
            }
        }
    }
}

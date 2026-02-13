use iced_core::{Background, Color, Theme};
use iced_widget::radio;
use serde::Deserialize;

use crate::color::HexColor;
use super::{BackgroundRaw, impl_merge};

// -- Layer 1: Serde raw types --

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct RadioFieldsRaw {
    background:   Option<BackgroundRaw>,
    dot_color:    Option<HexColor>,
    border_width: Option<f32>,
    border_color: Option<HexColor>,
    text_color:   Option<HexColor>,
}

impl_merge!(RadioFieldsRaw {
    background, dot_color,
    border_width, border_color,
    text_color,
});

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct RadioSection {
    #[serde(flatten)]
    base: RadioFieldsRaw,
    selected: Option<RadioFieldsRaw>,
    hovered: Option<RadioFieldsRaw>,
    disabled: Option<RadioFieldsRaw>,
    hovered_selected: Option<RadioFieldsRaw>,
    disabled_selected: Option<RadioFieldsRaw>,
}

// -- Layer 2: Resolution --

/// Cascade: base -> state -> status -> combined
fn cascade(
    base: RadioFieldsRaw,
    state: Option<&RadioFieldsRaw>,
    status: Option<&RadioFieldsRaw>,
    combined: Option<&RadioFieldsRaw>,
) -> radio::Style {
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

impl RadioSection {
    pub fn resolve(self) -> RadioStyle {
        let active_unselected = into_native(self.base);
        let active_selected = cascade(self.base, self.selected.as_ref(), None, None);
        let hovered_unselected = cascade(self.base, None, self.hovered.as_ref(), None);
        let hovered_selected = cascade(self.base, self.selected.as_ref(), self.hovered.as_ref(), self.hovered_selected.as_ref());

        RadioStyle {
            active_unselected,
            active_selected,
            hovered_unselected,
            hovered_selected,
        }
    }
}

fn into_native(f: RadioFieldsRaw) -> radio::Style {
    radio::Style {
        background: f.background.map(BackgroundRaw::into_background).unwrap_or(Background::Color(Color::TRANSPARENT)),
        dot_color: f.dot_color.map(|c| c.0).unwrap_or(Color::BLACK),
        border_width: f.border_width.unwrap_or(1.0),
        border_color: f.border_color.map(|c| c.0).unwrap_or(Color::BLACK),
        text_color: f.text_color.map(|c| c.0),
    }
}

// -- Layer 3: Public types --

/// Pre-resolved radio style with 4 variants (2 statuses x 2 states).
///
/// iced 0.14's `radio::Status` only has `Active` and `Hovered` — no `Disabled`.
/// The TOML `[radio.disabled]` and `[radio.disabled-selected]` sections are still
/// parsed but the resolved styles are not stored since iced cannot use them.
#[derive(Debug, Clone, Copy)]
pub struct RadioStyle {
    active_unselected:  radio::Style,
    active_selected:    radio::Style,
    hovered_unselected: radio::Style,
    hovered_selected:   radio::Style,
}

impl RadioStyle {
    /// Returns a closure suitable for passing to `.style()` on a radio widget.
    pub fn style_fn(&self) -> impl Fn(&Theme, radio::Status) -> radio::Style + Copy {
        let s = *self;
        move |_theme, status| match status {
            radio::Status::Active { is_selected } => {
                if is_selected { s.active_selected } else { s.active_unselected }
            }
            radio::Status::Hovered { is_selected } => {
                if is_selected { s.hovered_selected } else { s.hovered_unselected }
            }
        }
    }
}

use iced_core::Color;
use serde::Deserialize;

use crate::color::HexColor;
use super::impl_merge;

// -- Layer 1: Serde raw types --

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct RadioFieldsRaw {
    background:   Option<HexColor>,
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

fn cascade(
    base: RadioFieldsRaw,
    state: Option<&RadioFieldsRaw>,
    status: Option<&RadioFieldsRaw>,
    combined: Option<&RadioFieldsRaw>,
) -> RadioAppearance {
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

impl RadioSection {
    pub fn resolve(self) -> RadioStyle {
        let active_unselected = into_appearance(self.base);
        let active_selected = cascade(self.base, self.selected.as_ref(), None, None);
        let hovered_unselected = cascade(self.base, None, self.hovered.as_ref(), None);
        let hovered_selected = cascade(self.base, self.selected.as_ref(), self.hovered.as_ref(), self.hovered_selected.as_ref());
        let disabled_unselected = cascade(self.base, None, self.disabled.as_ref(), None);
        let disabled_selected = cascade(self.base, self.selected.as_ref(), self.disabled.as_ref(), self.disabled_selected.as_ref());

        RadioStyle {
            active_unselected,
            active_selected,
            hovered_unselected,
            hovered_selected,
            disabled_unselected,
            disabled_selected,
        }
    }
}

fn into_appearance(f: RadioFieldsRaw) -> RadioAppearance {
    RadioAppearance {
        background: f.background.map(|c| c.0).unwrap_or(Color::TRANSPARENT),
        dot_color: f.dot_color.map(|c| c.0).unwrap_or(Color::BLACK),
        border_width: f.border_width.unwrap_or(1.0),
        border_color: f.border_color.map(|c| c.0).unwrap_or(Color::BLACK),
        text_color: f.text_color.map(|c| c.0),
    }
}

// -- Layer 3: Public types --

/// Pre-resolved radio style with 6 variants (3 statuses × 2 states).
#[derive(Debug, Clone)]
pub struct RadioStyle {
    active_unselected:   RadioAppearance,
    active_selected:     RadioAppearance,
    hovered_unselected:  RadioAppearance,
    hovered_selected:    RadioAppearance,
    disabled_unselected: RadioAppearance,
    disabled_selected:   RadioAppearance,
}

impl RadioStyle {
    pub fn active(&self, is_selected: bool) -> &RadioAppearance {
        if is_selected { &self.active_selected } else { &self.active_unselected }
    }

    pub fn hovered(&self, is_selected: bool) -> &RadioAppearance {
        if is_selected { &self.hovered_selected } else { &self.hovered_unselected }
    }

    pub fn disabled(&self, is_selected: bool) -> &RadioAppearance {
        if is_selected { &self.disabled_selected } else { &self.disabled_unselected }
    }
}

/// Visual properties for a radio button. Fields mirror `iced_widget::radio::Style`.
#[derive(Debug, Clone, Copy)]
pub struct RadioAppearance {
    pub background: Color,
    pub dot_color: Color,
    pub border_width: f32,
    pub border_color: Color,
    pub text_color: Option<Color>,
}

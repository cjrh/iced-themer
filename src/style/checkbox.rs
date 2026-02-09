use iced_core::{Border, Color};
use serde::Deserialize;

use crate::color::HexColor;
use super::{RadiusRaw, impl_merge, resolve_border};

// -- Layer 1: Serde raw types --

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct CheckboxFieldsRaw {
    background:    Option<HexColor>,
    icon_color:    Option<HexColor>,
    border_width:  Option<f32>,
    border_color:  Option<HexColor>,
    border_radius: Option<RadiusRaw>,
    text_color:    Option<HexColor>,
}

impl_merge!(CheckboxFieldsRaw {
    background, icon_color,
    border_width, border_color, border_radius,
    text_color,
});

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct CheckboxSection {
    #[serde(flatten)]
    base: CheckboxFieldsRaw,
    checked: Option<CheckboxFieldsRaw>,
    hovered: Option<CheckboxFieldsRaw>,
    disabled: Option<CheckboxFieldsRaw>,
    hovered_checked: Option<CheckboxFieldsRaw>,
    disabled_checked: Option<CheckboxFieldsRaw>,
}

// -- Layer 2: Resolution --

/// Cascade: base → state → status → combined
fn cascade(
    base: CheckboxFieldsRaw,
    state: Option<&CheckboxFieldsRaw>,
    status: Option<&CheckboxFieldsRaw>,
    combined: Option<&CheckboxFieldsRaw>,
) -> CheckboxAppearance {
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

impl CheckboxSection {
    pub fn resolve(self) -> CheckboxStyle {
        let active_unchecked = into_appearance(self.base);
        let active_checked = cascade(self.base, self.checked.as_ref(), None, None);
        let hovered_unchecked = cascade(self.base, None, self.hovered.as_ref(), None);
        let hovered_checked = cascade(self.base, self.checked.as_ref(), self.hovered.as_ref(), self.hovered_checked.as_ref());
        let disabled_unchecked = cascade(self.base, None, self.disabled.as_ref(), None);
        let disabled_checked = cascade(self.base, self.checked.as_ref(), self.disabled.as_ref(), self.disabled_checked.as_ref());

        CheckboxStyle {
            active_unchecked,
            active_checked,
            hovered_unchecked,
            hovered_checked,
            disabled_unchecked,
            disabled_checked,
        }
    }
}

fn into_appearance(f: CheckboxFieldsRaw) -> CheckboxAppearance {
    CheckboxAppearance {
        background: f.background.map(|c| c.0).unwrap_or(Color::TRANSPARENT),
        icon_color: f.icon_color.map(|c| c.0).unwrap_or(Color::BLACK),
        border: resolve_border(f.border_width, f.border_color, f.border_radius),
        text_color: f.text_color.map(|c| c.0),
    }
}

// -- Layer 3: Public types --

/// Pre-resolved checkbox style with 6 variants (3 statuses × 2 states).
#[derive(Debug, Clone)]
pub struct CheckboxStyle {
    active_unchecked:   CheckboxAppearance,
    active_checked:     CheckboxAppearance,
    hovered_unchecked:  CheckboxAppearance,
    hovered_checked:    CheckboxAppearance,
    disabled_unchecked: CheckboxAppearance,
    disabled_checked:   CheckboxAppearance,
}

impl CheckboxStyle {
    pub fn active(&self, is_checked: bool) -> &CheckboxAppearance {
        if is_checked { &self.active_checked } else { &self.active_unchecked }
    }

    pub fn hovered(&self, is_checked: bool) -> &CheckboxAppearance {
        if is_checked { &self.hovered_checked } else { &self.hovered_unchecked }
    }

    pub fn disabled(&self, is_checked: bool) -> &CheckboxAppearance {
        if is_checked { &self.disabled_checked } else { &self.disabled_unchecked }
    }
}

/// Visual properties for a checkbox. Fields mirror `iced_widget::checkbox::Style`.
#[derive(Debug, Clone, Copy)]
pub struct CheckboxAppearance {
    pub background: Color,
    pub icon_color: Color,
    pub border: Border,
    pub text_color: Option<Color>,
}

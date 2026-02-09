use iced_core::{Background, Color, Theme};
use iced_widget::checkbox;
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

/// Cascade: base -> state -> status -> combined
fn cascade(
    base: CheckboxFieldsRaw,
    state: Option<&CheckboxFieldsRaw>,
    status: Option<&CheckboxFieldsRaw>,
    combined: Option<&CheckboxFieldsRaw>,
) -> checkbox::Style {
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

impl CheckboxSection {
    pub fn resolve(self) -> CheckboxStyle {
        let active_unchecked = into_native(self.base);
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

fn into_native(f: CheckboxFieldsRaw) -> checkbox::Style {
    checkbox::Style {
        background: Background::Color(f.background.map(|c| c.0).unwrap_or(Color::TRANSPARENT)),
        icon_color: f.icon_color.map(|c| c.0).unwrap_or(Color::BLACK),
        border: resolve_border(f.border_width, f.border_color, f.border_radius),
        text_color: f.text_color.map(|c| c.0),
    }
}

// -- Layer 3: Public types --

/// Pre-resolved checkbox style with 6 variants (3 statuses x 2 states).
#[derive(Debug, Clone, Copy)]
pub struct CheckboxStyle {
    active_unchecked:   checkbox::Style,
    active_checked:     checkbox::Style,
    hovered_unchecked:  checkbox::Style,
    hovered_checked:    checkbox::Style,
    disabled_unchecked: checkbox::Style,
    disabled_checked:   checkbox::Style,
}

impl CheckboxStyle {
    /// Returns a closure suitable for passing to `.style()` on a checkbox widget.
    pub fn style_fn(&self) -> impl Fn(&Theme, checkbox::Status) -> checkbox::Style + Copy {
        let s = *self;
        move |_theme, status| match status {
            checkbox::Status::Active { is_checked } => {
                if is_checked { s.active_checked } else { s.active_unchecked }
            }
            checkbox::Status::Hovered { is_checked } => {
                if is_checked { s.hovered_checked } else { s.hovered_unchecked }
            }
            checkbox::Status::Disabled { is_checked } => {
                if is_checked { s.disabled_checked } else { s.disabled_unchecked }
            }
        }
    }
}

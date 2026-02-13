use iced_core::Theme;
use iced_widget::container;
use serde::Deserialize;

use crate::color::HexColor;
use super::{BackgroundRaw, RadiusRaw, impl_merge, resolve_border, resolve_shadow};

// -- Layer 1: Serde raw types --

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct ContainerFieldsRaw {
    background:         Option<BackgroundRaw>,
    text_color:         Option<HexColor>,
    border_width:       Option<f32>,
    border_color:       Option<HexColor>,
    border_radius:      Option<RadiusRaw>,
    shadow_color:       Option<HexColor>,
    shadow_offset_x:    Option<f32>,
    shadow_offset_y:    Option<f32>,
    shadow_blur_radius: Option<f32>,
}

impl_merge!(ContainerFieldsRaw {
    background, text_color,
    border_width, border_color, border_radius,
    shadow_color, shadow_offset_x, shadow_offset_y, shadow_blur_radius,
});

/// Top-level `[container]` section. No status sub-tables.
#[derive(Deserialize)]
pub(crate) struct ContainerSection {
    #[serde(flatten)]
    base: ContainerFieldsRaw,
}

// -- Layer 2: Resolution --

impl ContainerSection {
    pub fn resolve(self) -> ContainerStyle {
        ContainerStyle(into_native(self.base))
    }
}

fn into_native(f: ContainerFieldsRaw) -> container::Style {
    container::Style {
        background: f.background.map(BackgroundRaw::into_background),
        text_color: f.text_color.map(|c| c.0),
        border: resolve_border(f.border_width, f.border_color, f.border_radius),
        shadow: resolve_shadow(f.shadow_color, f.shadow_offset_x, f.shadow_offset_y, f.shadow_blur_radius),
        snap: false,
    }
}

// -- Layer 3: Public types --

/// Pre-resolved container style.
#[derive(Debug, Clone, Copy)]
pub struct ContainerStyle(container::Style);

impl ContainerStyle {
    /// Returns a closure suitable for passing to `.style()` on a container widget.
    pub fn style_fn(&self) -> impl Fn(&Theme) -> container::Style + Copy {
        let s = self.0;
        move |_theme| s
    }
}

use iced_core::{Background, Color, Theme};
use iced_widget::progress_bar;
use serde::Deserialize;

use crate::color::HexColor;
use super::{BackgroundRaw, RadiusRaw, impl_merge, resolve_border};

// -- Layer 1: Serde raw types --

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct ProgressBarFieldsRaw {
    background:    Option<BackgroundRaw>,
    bar:           Option<BackgroundRaw>,
    border_width:  Option<f32>,
    border_color:  Option<HexColor>,
    border_radius: Option<RadiusRaw>,
}

impl_merge!(ProgressBarFieldsRaw {
    background, bar,
    border_width, border_color, border_radius,
});

/// Top-level `[progress-bar]` section. No status sub-tables.
#[derive(Deserialize)]
pub(crate) struct ProgressBarSection {
    #[serde(flatten)]
    base: ProgressBarFieldsRaw,
}

// -- Layer 2: Resolution --

impl ProgressBarSection {
    pub fn resolve(self) -> ProgressBarStyle {
        ProgressBarStyle(into_native(self.base))
    }
}

fn into_native(f: ProgressBarFieldsRaw) -> progress_bar::Style {
    progress_bar::Style {
        background: f.background.map(BackgroundRaw::into_background).unwrap_or(Background::Color(Color::TRANSPARENT)),
        bar: f.bar.map(BackgroundRaw::into_background).unwrap_or(Background::Color(Color::BLACK)),
        border: resolve_border(f.border_width, f.border_color, f.border_radius),
    }
}

// -- Layer 3: Public types --

/// Pre-resolved progress bar style.
#[derive(Debug, Clone, Copy)]
pub struct ProgressBarStyle(progress_bar::Style);

impl ProgressBarStyle {
    /// Returns a closure suitable for passing to `.style()` on a progress bar widget.
    pub fn style_fn(&self) -> impl Fn(&Theme) -> progress_bar::Style + Copy + 'static {
        let s = self.0;
        move |_theme| s
    }
}

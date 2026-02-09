use iced_core::{Background, Border, Color};
use serde::Deserialize;

use crate::color::HexColor;
use super::{RadiusRaw, impl_merge, resolve_border};

// -- Layer 1: Serde raw types --

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct ProgressBarFieldsRaw {
    background:    Option<HexColor>,
    bar:           Option<HexColor>,
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
        ProgressBarStyle(into_appearance(self.base))
    }
}

fn into_appearance(f: ProgressBarFieldsRaw) -> ProgressBarAppearance {
    let bg_color = f.background.map(|c| c.0).unwrap_or(Color::TRANSPARENT);
    let bar_color = f.bar.map(|c| c.0).unwrap_or(Color::BLACK);

    ProgressBarAppearance {
        background: Background::Color(bg_color),
        bar: Background::Color(bar_color),
        border: resolve_border(f.border_width, f.border_color, f.border_radius),
    }
}

// -- Layer 3: Public types --

/// Pre-resolved progress bar style. Mirrors `iced_widget::progress_bar::Style`.
#[derive(Debug, Clone)]
pub struct ProgressBarStyle(ProgressBarAppearance);

impl ProgressBarStyle {
    pub fn appearance(&self) -> &ProgressBarAppearance {
        &self.0
    }
}

/// Visual properties for a progress bar.
#[derive(Debug, Clone, Copy)]
pub struct ProgressBarAppearance {
    pub background: Background,
    pub bar: Background,
    pub border: Border,
}

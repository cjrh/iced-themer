//! Per-widget style types parsed from TOML.
//!
//! Each widget module provides a `Style` type with pre-resolved appearances for
//! every status variant. Users match on iced's status enums and copy fields into
//! the corresponding `iced_widget` style structs.

mod button;
mod checkbox;
mod container;
mod progress_bar;
mod radio;
mod slider;
mod text_input;
mod toggler;

pub use button::{ButtonAppearance, ButtonStyle};
pub use checkbox::{CheckboxAppearance, CheckboxStyle};
pub use container::{ContainerAppearance, ContainerStyle};
pub use progress_bar::{ProgressBarAppearance, ProgressBarStyle};
pub use radio::{RadioAppearance, RadioStyle};
pub use slider::{HandleShapeKind, SliderAppearance, SliderStyle};
pub use text_input::{TextInputAppearance, TextInputStyle};
pub use toggler::{TogglerAppearance, TogglerStyle};

pub(crate) use button::ButtonSection;
pub(crate) use checkbox::CheckboxSection;
pub(crate) use container::ContainerSection;
pub(crate) use progress_bar::ProgressBarSection;
pub(crate) use radio::RadioSection;
pub(crate) use slider::SliderSection;
pub(crate) use text_input::TextInputSection;
pub(crate) use toggler::TogglerSection;

use iced_core::Border;
use serde::Deserialize;

use crate::color::HexColor;

/// Flexible border-radius: a single `f32` for uniform corners, or `[f32; 4]`
/// for `[top-left, top-right, bottom-right, bottom-left]`.
#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(untagged)]
pub(crate) enum RadiusRaw {
    Uniform(f32),
    PerCorner([f32; 4]),
}

impl RadiusRaw {
    pub fn into_radius(self) -> iced_core::border::Radius {
        match self {
            RadiusRaw::Uniform(v) => v.into(),
            RadiusRaw::PerCorner([tl, tr, br, bl]) => iced_core::border::Radius {
                top_left: tl,
                top_right: tr,
                bottom_right: br,
                bottom_left: bl,
            },
        }
    }
}

/// Resolve border fields from raw Option values, falling back to iced defaults.
pub(crate) fn resolve_border(
    width: Option<f32>,
    color: Option<HexColor>,
    radius: Option<RadiusRaw>,
) -> Border {
    Border {
        color: color.map(|c| c.0).unwrap_or(iced_core::Color::TRANSPARENT),
        width: width.unwrap_or(0.0),
        radius: radius.map(RadiusRaw::into_radius).unwrap_or(0.0.into()),
    }
}

/// Resolve shadow fields from raw Option values.
pub(crate) fn resolve_shadow(
    color: Option<HexColor>,
    offset_x: Option<f32>,
    offset_y: Option<f32>,
    blur_radius: Option<f32>,
) -> iced_core::Shadow {
    iced_core::Shadow {
        color: color.map(|c| c.0).unwrap_or(iced_core::Color::TRANSPARENT),
        offset: iced_core::Vector::new(offset_x.unwrap_or(0.0), offset_y.unwrap_or(0.0)),
        blur_radius: blur_radius.unwrap_or(0.0),
    }
}

/// Generates a `merge` method for structs whose fields are all `Option<T: Copy>`.
///
/// `merge(self, over)` returns a new instance where each field takes `over`'s
/// value if present, otherwise `self`'s value. This powers the status-override
/// cascade: base fields are overridden by more-specific sub-table fields.
macro_rules! impl_merge {
    ($ty:ty { $($field:ident),+ $(,)? }) => {
        impl $ty {
            #[allow(dead_code)]
            pub(crate) fn merge(self, over: &Self) -> Self {
                Self {
                    $( $field: over.$field.or(self.$field), )+
                }
            }
        }
    };
}

pub(crate) use impl_merge;

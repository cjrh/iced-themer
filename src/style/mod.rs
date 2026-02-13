//! Per-widget style types parsed from TOML.
//!
//! Each widget module provides a `Style` type with pre-resolved native
//! `iced_widget` styles for every status variant. Call `style_fn()` and pass
//! the result directly to the widget's `.style()` builder method.

mod button;
mod checkbox;
mod container;
mod progress_bar;
mod radio;
mod slider;
mod text_input;
mod toggler;

pub use button::ButtonStyle;
pub use checkbox::CheckboxStyle;
pub use container::ContainerStyle;
pub use progress_bar::ProgressBarStyle;
pub use radio::RadioStyle;
pub use slider::SliderStyle;
pub use text_input::TextInputStyle;
pub use toggler::TogglerStyle;

pub(crate) use button::ButtonSection;
pub(crate) use checkbox::CheckboxSection;
pub(crate) use container::ContainerSection;
pub(crate) use progress_bar::ProgressBarSection;
pub(crate) use radio::RadioSection;
pub(crate) use slider::SliderSection;
pub(crate) use text_input::TextInputSection;
pub(crate) use toggler::TogglerSection;

use iced_core::{Background, Border, Degrees};
use iced_core::gradient::Linear;
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

/// A background that is either a solid color or a linear gradient.
///
/// Strings deserialize as solid colors via `HexColor`; tables with `angle` and
/// `stops` fields deserialize as gradients. This mirrors `iced_core::Background`
/// but uses serde-friendly types.
#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(untagged)]
pub(crate) enum BackgroundRaw {
    Color(HexColor),
    Gradient(GradientRaw),
}

impl BackgroundRaw {
    pub fn into_background(self) -> Background {
        match self {
            BackgroundRaw::Color(c) => Background::Color(c.0),
            BackgroundRaw::Gradient(g) => g.into_background(),
        }
    }
}

/// A single color stop in a gradient: an offset in `0.0..=1.0` and a color.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ColorStopEntry {
    pub offset: f32,
    pub color: HexColor,
}

/// A linear gradient with an angle (in degrees) and up to 8 color stops.
///
/// Uses a fixed-size array to preserve `Copy` throughout the style system.
/// A custom `Deserialize` reads a TOML vec and packs it into the array,
/// validating the stop count and offset range.
#[derive(Clone, Copy, Debug)]
pub(crate) struct GradientRaw {
    pub angle: f32,
    pub stops: [Option<ColorStopEntry>; 8],
}

impl GradientRaw {
    fn into_background(self) -> Background {
        let mut linear = Linear::new(Degrees(self.angle));
        for stop in self.stops.into_iter().flatten() {
            linear = linear.add_stop(stop.offset, stop.color.0);
        }
        Background::Gradient(iced_core::Gradient::Linear(linear))
    }
}

impl<'de> Deserialize<'de> for GradientRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct StopHelper {
            offset: f32,
            color: HexColor,
        }

        #[derive(Deserialize)]
        struct GradientHelper {
            angle: f32,
            stops: Vec<StopHelper>,
        }

        let helper = GradientHelper::deserialize(deserializer)?;

        if helper.stops.len() > 8 {
            return Err(serde::de::Error::custom(format!(
                "gradient supports at most 8 color stops, got {}",
                helper.stops.len()
            )));
        }

        let mut arr = [None; 8];
        for (i, s) in helper.stops.into_iter().enumerate() {
            if !(0.0..=1.0).contains(&s.offset) {
                return Err(serde::de::Error::custom(format!(
                    "color stop offset must be in 0.0..=1.0, got {}",
                    s.offset
                )));
            }
            arr[i] = Some(ColorStopEntry {
                offset: s.offset,
                color: s.color,
            });
        }

        Ok(GradientRaw {
            angle: helper.angle,
            stops: arr,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::Background;

    /// Helper: wraps a TOML value in a `bg = ...` key so we can deserialize
    /// `BackgroundRaw` from a standalone value via a wrapper struct.
    #[derive(Deserialize)]
    struct Wrapper {
        bg: BackgroundRaw,
    }

    fn parse_bg(toml_str: &str) -> Result<BackgroundRaw, toml::de::Error> {
        toml::from_str::<Wrapper>(toml_str).map(|w| w.bg)
    }

    #[test]
    fn background_raw_parses_solid_color() {
        let raw = parse_bg(r##"bg = "#FF0000""##).unwrap();
        assert!(matches!(raw, BackgroundRaw::Color(_)));
    }

    #[test]
    fn background_raw_parses_gradient() {
        let raw = parse_bg(r##"
            [bg]
            angle = 45.0
            stops = [
                { offset = 0.0, color = "#ff0000" },
                { offset = 1.0, color = "#0000ff" },
            ]
        "##).unwrap();
        match raw {
            BackgroundRaw::Gradient(g) => {
                assert!((g.angle - 45.0).abs() < f32::EPSILON);
                assert!(g.stops[0].is_some());
                assert!(g.stops[1].is_some());
                assert!(g.stops[2].is_none());
            }
            _ => panic!("expected Gradient variant"),
        }
    }

    #[test]
    fn gradient_rejects_more_than_8_stops() {
        let result: Result<GradientRaw, _> = toml::from_str(r##"
            angle = 0.0
            stops = [
                { offset = 0.0, color = "#000000" },
                { offset = 0.1, color = "#111111" },
                { offset = 0.2, color = "#222222" },
                { offset = 0.3, color = "#333333" },
                { offset = 0.4, color = "#444444" },
                { offset = 0.5, color = "#555555" },
                { offset = 0.6, color = "#666666" },
                { offset = 0.7, color = "#777777" },
                { offset = 0.8, color = "#888888" },
            ]
        "##);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("at most 8"), "error was: {err}");
    }

    #[test]
    fn gradient_rejects_offset_out_of_range() {
        let result: Result<GradientRaw, _> = toml::from_str(r##"
            angle = 0.0
            stops = [
                { offset = 0.0, color = "#000000" },
                { offset = 1.5, color = "#ffffff" },
            ]
        "##);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("0.0..=1.0"), "error was: {err}");
    }

    #[test]
    fn gradient_accepts_8_stops() {
        let raw: GradientRaw = toml::from_str(r##"
            angle = 90.0
            stops = [
                { offset = 0.0,   color = "#000000" },
                { offset = 0.143, color = "#111111" },
                { offset = 0.286, color = "#222222" },
                { offset = 0.429, color = "#333333" },
                { offset = 0.571, color = "#444444" },
                { offset = 0.714, color = "#555555" },
                { offset = 0.857, color = "#666666" },
                { offset = 1.0,   color = "#777777" },
            ]
        "##).unwrap();
        assert!(raw.stops.iter().all(|s| s.is_some()));
    }

    #[test]
    fn solid_color_converts_to_background_color() {
        let raw = parse_bg(r##"bg = "#ff0000""##).unwrap();
        let bg = raw.into_background();
        match bg {
            Background::Color(c) => {
                assert!((c.r - 1.0).abs() < 0.01);
                assert!(c.g.abs() < 0.01);
                assert!(c.b.abs() < 0.01);
            }
            _ => panic!("expected Background::Color"),
        }
    }

    #[test]
    fn gradient_converts_to_background_gradient() {
        let raw = parse_bg(r##"
            [bg]
            angle = 90.0
            stops = [
                { offset = 0.0, color = "#ff0000" },
                { offset = 1.0, color = "#0000ff" },
            ]
        "##).unwrap();
        let bg = raw.into_background();
        match bg {
            Background::Gradient(iced_core::Gradient::Linear(linear)) => {
                assert!(linear.stops[0].is_some());
                assert!(linear.stops[1].is_some());
                let s0 = linear.stops[0].unwrap();
                assert!(s0.offset.abs() < f32::EPSILON);
                assert!((s0.color.r - 1.0).abs() < 0.01);
            }
            _ => panic!("expected Background::Gradient(Linear(..))"),
        }
    }
}

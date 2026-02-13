use iced_core::{Background, Border, Color, Theme};
use iced_widget::slider;
use serde::Deserialize;

use crate::color::HexColor;
use super::{BackgroundRaw, RadiusRaw, impl_merge};

// -- Layer 1: Serde raw types --

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct SliderFieldsRaw {
    rail_color_1:         Option<HexColor>,
    rail_color_2:         Option<HexColor>,
    rail_width:           Option<f32>,
    rail_border_radius:   Option<RadiusRaw>,
    handle_shape:         Option<HandleShapeKindRaw>,
    handle_radius:        Option<f32>,
    handle_width:         Option<f32>,
    handle_border_radius: Option<RadiusRaw>,
    handle_background:    Option<BackgroundRaw>,
    handle_border_width:  Option<f32>,
    handle_border_color:  Option<HexColor>,
}

impl_merge!(SliderFieldsRaw {
    rail_color_1, rail_color_2, rail_width, rail_border_radius,
    handle_shape, handle_radius, handle_width, handle_border_radius,
    handle_background, handle_border_width, handle_border_color,
});

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default, rename_all = "kebab-case")]
pub(crate) struct SliderSection {
    #[serde(flatten)]
    base: SliderFieldsRaw,
    hovered: Option<SliderFieldsRaw>,
    dragged: Option<SliderFieldsRaw>,
}

/// Internal serde mirror for handle shape kinds.
#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum HandleShapeKindRaw {
    Circle,
    Rectangle,
}

// -- Layer 2: Resolution --

impl SliderSection {
    pub fn resolve(self) -> SliderStyle {
        let active = into_native(self.base);
        let hovered = resolve_status(self.base, self.hovered.as_ref());
        let dragged = resolve_status(self.base, self.dragged.as_ref());

        SliderStyle { active, hovered, dragged }
    }
}

fn resolve_status(base: SliderFieldsRaw, status: Option<&SliderFieldsRaw>) -> slider::Style {
    match status {
        Some(over) => into_native(base.merge(over)),
        None => into_native(base),
    }
}

fn into_native(f: SliderFieldsRaw) -> slider::Style {
    let rail_border_radius = f.rail_border_radius.map(RadiusRaw::into_radius).unwrap_or(0.0.into());

    let handle_shape = match f.handle_shape.unwrap_or(HandleShapeKindRaw::Circle) {
        HandleShapeKindRaw::Circle => slider::HandleShape::Circle {
            radius: f.handle_radius.unwrap_or(7.0),
        },
        HandleShapeKindRaw::Rectangle => slider::HandleShape::Rectangle {
            width: f.handle_width.unwrap_or(8.0) as u16,
            border_radius: f.handle_border_radius.map(RadiusRaw::into_radius).unwrap_or(2.0.into()),
        },
    };

    slider::Style {
        rail: slider::Rail {
            backgrounds: (
                Background::Color(f.rail_color_1.map(|c| c.0).unwrap_or(Color::BLACK)),
                Background::Color(f.rail_color_2.map(|c| c.0).unwrap_or(Color::TRANSPARENT)),
            ),
            width: f.rail_width.unwrap_or(4.0),
            border: Border {
                radius: rail_border_radius,
                ..Default::default()
            },
        },
        handle: slider::Handle {
            shape: handle_shape,
            background: f.handle_background.map(BackgroundRaw::into_background).unwrap_or(Background::Color(Color::BLACK)),
            border_width: f.handle_border_width.unwrap_or(0.0),
            border_color: f.handle_border_color.map(|c| c.0).unwrap_or(Color::TRANSPARENT),
        },
    }
}

// -- Layer 3: Public types --

/// Pre-resolved slider style with a native `iced_widget` style for each status variant.
#[derive(Debug, Clone, Copy)]
pub struct SliderStyle {
    active:  slider::Style,
    hovered: slider::Style,
    dragged: slider::Style,
}

impl SliderStyle {
    /// Returns a closure suitable for passing to `.style()` on a slider widget.
    pub fn style_fn(&self) -> impl Fn(&Theme, slider::Status) -> slider::Style + Copy {
        let s = *self;
        move |_theme, status| match status {
            slider::Status::Active  => s.active,
            slider::Status::Hovered => s.hovered,
            slider::Status::Dragged => s.dragged,
        }
    }
}

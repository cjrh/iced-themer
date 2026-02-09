use iced_core::{border::Radius, Background, Border, Color};
use serde::Deserialize;

use crate::color::HexColor;
use super::{RadiusRaw, impl_merge};

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
    handle_background:    Option<HexColor>,
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
        let active = into_appearance(self.base);
        let hovered = resolve_status(self.base, self.hovered.as_ref());
        let dragged = resolve_status(self.base, self.dragged.as_ref());

        SliderStyle { active, hovered, dragged }
    }
}

fn resolve_status(base: SliderFieldsRaw, status: Option<&SliderFieldsRaw>) -> SliderAppearance {
    match status {
        Some(over) => into_appearance(base.merge(over)),
        None => into_appearance(base),
    }
}

fn into_appearance(f: SliderFieldsRaw) -> SliderAppearance {
    let rail_border_radius = f.rail_border_radius.map(RadiusRaw::into_radius).unwrap_or(0.0.into());

    let handle_shape = match f.handle_shape.unwrap_or(HandleShapeKindRaw::Circle) {
        HandleShapeKindRaw::Circle => HandleShapeKind::Circle {
            radius: f.handle_radius.unwrap_or(7.0),
        },
        HandleShapeKindRaw::Rectangle => HandleShapeKind::Rectangle {
            width: f.handle_width.unwrap_or(8.0) as u16,
            border_radius: f.handle_border_radius.map(RadiusRaw::into_radius).unwrap_or(2.0.into()),
        },
    };

    SliderAppearance {
        rail_color_1: f.rail_color_1.map(|c| c.0).unwrap_or(Color::BLACK),
        rail_color_2: f.rail_color_2.map(|c| c.0).unwrap_or(Color::TRANSPARENT),
        rail_width: f.rail_width.unwrap_or(4.0),
        rail_border_radius,
        handle_shape,
        handle_background: Background::Color(
            f.handle_background.map(|c| c.0).unwrap_or(Color::BLACK),
        ),
        handle_border: Border {
            color: f.handle_border_color.map(|c| c.0).unwrap_or(Color::TRANSPARENT),
            width: f.handle_border_width.unwrap_or(0.0),
            radius: 0.0.into(),
        },
    }
}

// -- Layer 3: Public types --

/// Pre-resolved slider style with an appearance for each status variant.
#[derive(Debug, Clone)]
pub struct SliderStyle {
    active:  SliderAppearance,
    hovered: SliderAppearance,
    dragged: SliderAppearance,
}

impl SliderStyle {
    pub fn active(&self) -> &SliderAppearance {
        &self.active
    }

    pub fn hovered(&self) -> &SliderAppearance {
        &self.hovered
    }

    pub fn dragged(&self) -> &SliderAppearance {
        &self.dragged
    }
}

/// Handle shape enumeration, mirroring `iced_widget::slider::HandleShape`.
#[derive(Debug, Clone, Copy)]
pub enum HandleShapeKind {
    Circle { radius: f32 },
    Rectangle { width: u16, border_radius: Radius },
}

/// Visual properties for a slider. Fields mirror `iced_widget::slider::Style`.
#[derive(Debug, Clone, Copy)]
pub struct SliderAppearance {
    pub rail_color_1: Color,
    pub rail_color_2: Color,
    pub rail_width: f32,
    pub rail_border_radius: Radius,
    pub handle_shape: HandleShapeKind,
    pub handle_background: Background,
    pub handle_border: Border,
}

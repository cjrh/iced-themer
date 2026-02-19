use iced_widget::{Button, Checkbox, Container, ProgressBar, Radio, Slider, TextInput, Toggler};

use crate::style::{
    ButtonStyle, CheckboxStyle, ContainerStyle, ProgressBarStyle, RadioStyle, SliderStyle,
    TextInputStyle, TogglerStyle,
};

/// Applies an optional theme style to a widget inline in the builder chain.
///
/// Returns the widget unchanged when `style` is `None`, and calls `.style()`
/// when `Some`. Import this trait once to use `.themed()` on any iced widget
/// that has a corresponding iced-themer style type.
///
/// # Example
///
/// ```no_run
/// use iced::widget::slider;
/// use iced_themer::{ThemeConfig, Themed};
///
/// # let config = ThemeConfig::from_file("theme.toml").unwrap();
/// # let value = 50.0f32;
/// # let on_change = |_: f32| ();
/// let s = slider(0.0..=100.0, value, on_change)
///     .themed(config.slider());
/// ```
pub trait Themed<S>: Sized {
    fn themed(self, style: Option<&S>) -> Self;
}

impl<'a, T, M> Themed<SliderStyle> for Slider<'a, T, M>
where
    T: Copy + From<u8> + PartialOrd,
    M: Clone,
{
    fn themed(self, style: Option<&SliderStyle>) -> Self {
        match style {
            Some(s) => self.style(s.style_fn()),
            None => self,
        }
    }
}

impl<'a, M, R> Themed<ButtonStyle> for Button<'a, M, iced_core::Theme, R>
where
    R: iced_core::Renderer,
{
    fn themed(self, style: Option<&ButtonStyle>) -> Self {
        match style {
            Some(s) => self.style(s.style_fn()),
            None => self,
        }
    }
}

impl<'a, M, R> Themed<ContainerStyle> for Container<'a, M, iced_core::Theme, R>
where
    R: iced_core::Renderer,
{
    fn themed(self, style: Option<&ContainerStyle>) -> Self {
        match style {
            Some(s) => self.style(s.style_fn()),
            None => self,
        }
    }
}

impl<'a, M, R> Themed<TextInputStyle> for TextInput<'a, M, iced_core::Theme, R>
where
    M: Clone,
    R: iced_core::text::Renderer,
{
    fn themed(self, style: Option<&TextInputStyle>) -> Self {
        match style {
            Some(s) => self.style(s.style_fn()),
            None => self,
        }
    }
}

impl<'a, M, R> Themed<CheckboxStyle> for Checkbox<'a, M, iced_core::Theme, R>
where
    R: iced_core::text::Renderer,
{
    fn themed(self, style: Option<&CheckboxStyle>) -> Self {
        match style {
            Some(s) => self.style(s.style_fn()),
            None => self,
        }
    }
}

impl<'a, M, R> Themed<TogglerStyle> for Toggler<'a, M, iced_core::Theme, R>
where
    R: iced_core::text::Renderer,
{
    fn themed(self, style: Option<&TogglerStyle>) -> Self {
        match style {
            Some(s) => self.style(s.style_fn()),
            None => self,
        }
    }
}

impl<'a, M, R> Themed<RadioStyle> for Radio<'a, M, iced_core::Theme, R>
where
    M: Clone,
    R: iced_core::text::Renderer,
{
    fn themed(self, style: Option<&RadioStyle>) -> Self {
        match style {
            Some(s) => self.style(s.style_fn()),
            None => self,
        }
    }
}

impl<'a> Themed<ProgressBarStyle> for ProgressBar<'a> {
    fn themed(self, style: Option<&ProgressBarStyle>) -> Self {
        match style {
            Some(s) => self.style(s.style_fn()),
            None => self,
        }
    }
}

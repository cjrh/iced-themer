use iced::widget::{
    button, checkbox, column, container, progress_bar, radio, row, slider, text, text_input,
    toggler,
};
use iced::{Background, Element, Length, Theme};
use iced_themer::style::{
    ButtonStyle, CheckboxStyle, ContainerStyle, ProgressBarStyle, SliderStyle, TextInputStyle,
    TogglerStyle,
};
use iced_themer::ThemeConfig;

fn main() -> iced::Result {
    let config = ThemeConfig::from_file("example/theme.toml").expect("failed to load theme.toml");

    let theme = config.theme();
    let font = config.font();

    let state = AppState {
        button_style: config.button().cloned(),
        container_style: config.container().cloned(),
        text_input_style: config.text_input().cloned(),
        checkbox_style: config.checkbox().cloned(),
        toggler_style: config.toggler().cloned(),
        slider_style: config.slider().cloned(),
        progress_bar_style: config.progress_bar().cloned(),
    };

    let app = iced::application(move || App::new(state.clone()), App::update, App::view)
        .title("iced-themer Demo")
        .theme(move |_: &App| -> Theme { theme.clone() });

    match font {
        Some(f) => app.default_font(f).run(),
        None => app.run(),
    }
}

#[derive(Clone)]
struct AppState {
    button_style: Option<ButtonStyle>,
    container_style: Option<ContainerStyle>,
    text_input_style: Option<TextInputStyle>,
    checkbox_style: Option<CheckboxStyle>,
    toggler_style: Option<TogglerStyle>,
    slider_style: Option<SliderStyle>,
    progress_bar_style: Option<ProgressBarStyle>,
}

struct App {
    input_value: String,
    is_checked: bool,
    is_toggled: bool,
    slider_value: f32,
    selected_option: Option<&'static str>,
    styles: AppState,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ButtonPressed,
    CheckboxToggled(bool),
    TogglerToggled(bool),
    SliderChanged(f32),
    RadioSelected(&'static str),
}

impl App {
    fn new(styles: AppState) -> Self {
        Self {
            input_value: String::new(),
            is_checked: false,
            is_toggled: false,
            slider_value: 50.0,
            selected_option: None,
            styles,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => self.input_value.clear(),
            Message::CheckboxToggled(value) => self.is_checked = value,
            Message::TogglerToggled(value) => self.is_toggled = value,
            Message::SliderChanged(value) => self.slider_value = value,
            Message::RadioSelected(value) => self.selected_option = Some(value),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let heading = text("iced-themer Demo").size(28);

        // Text input with themed style
        let mut input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged);
        if let Some(s) = &self.styles.text_input_style {
            let s = s.clone();
            input = input.style(move |_theme, status| {
                let a = match status {
                    text_input::Status::Active => s.active(),
                    text_input::Status::Hovered => s.active(),
                    text_input::Status::Focused { .. } => s.focused(),
                    text_input::Status::Disabled => s.disabled(),
                };
                text_input::Style {
                    background: a.background,
                    border: a.border,
                    icon: a.icon_color,
                    placeholder: a.placeholder_color,
                    value: a.value_color,
                    selection: a.selection_color,
                }
            });
        }

        // Button with themed style
        let mut btn = button("Clear").on_press(Message::ButtonPressed);
        if let Some(s) = &self.styles.button_style {
            let s = s.clone();
            btn = btn.style(move |_theme, status| {
                let a = match status {
                    button::Status::Active => s.active(),
                    button::Status::Hovered => s.hovered(),
                    button::Status::Pressed => s.pressed(),
                    button::Status::Disabled => s.disabled(),
                };
                button::Style {
                    background: a.background,
                    text_color: a.text_color,
                    border: a.border,
                    shadow: a.shadow,
                    snap: false,
                }
            });
        }

        // Checkbox with themed style
        let mut check = checkbox(self.is_checked)
            .label("Enable feature")
            .on_toggle(Message::CheckboxToggled);
        if let Some(s) = &self.styles.checkbox_style {
            let s = s.clone();
            check = check.style(move |_theme, status| {
                let a = match status {
                    checkbox::Status::Active { is_checked } => s.active(is_checked),
                    checkbox::Status::Hovered { is_checked } => s.hovered(is_checked),
                    checkbox::Status::Disabled { is_checked } => s.disabled(is_checked),
                };
                checkbox::Style {
                    background: Background::Color(a.background),
                    icon_color: a.icon_color,
                    border: a.border,
                    text_color: a.text_color,
                }
            });
        }

        // Toggler with themed style
        let mut tog = toggler(self.is_toggled)
            .label("Dark mode")
            .on_toggle(Message::TogglerToggled);
        if let Some(s) = &self.styles.toggler_style {
            let s = s.clone();
            tog = tog.style(move |_theme, status| {
                let a = match status {
                    toggler::Status::Active { is_toggled } => s.active(is_toggled),
                    toggler::Status::Hovered { is_toggled } => s.hovered(is_toggled),
                    toggler::Status::Disabled { is_toggled } => s.disabled(is_toggled),
                };
                toggler::Style {
                    background: Background::Color(a.background),
                    foreground: Background::Color(a.foreground),
                    background_border_width: a.background_border_width,
                    background_border_color: a.background_border_color,
                    foreground_border_width: a.foreground_border_width,
                    foreground_border_color: a.foreground_border_color,
                    border_radius: a.border_radius.map(|r| r.into()),
                    text_color: a.text_color,
                    padding_ratio: 0.36,
                }
            });
        }

        // Slider with themed style
        let mut sld = slider(0.0..=100.0, self.slider_value, Message::SliderChanged);
        if let Some(s) = &self.styles.slider_style {
            let s = s.clone();
            sld = sld.style(move |_theme, status| {
                let a = match status {
                    slider::Status::Active => s.active(),
                    slider::Status::Hovered => s.hovered(),
                    slider::Status::Dragged => s.dragged(),
                };

                let handle_shape = match a.handle_shape {
                    iced_themer::style::HandleShapeKind::Circle { radius } => {
                        slider::HandleShape::Circle { radius }
                    }
                    iced_themer::style::HandleShapeKind::Rectangle {
                        width,
                        border_radius,
                    } => slider::HandleShape::Rectangle {
                        width,
                        border_radius,
                    },
                };

                slider::Style {
                    rail: slider::Rail {
                        backgrounds: (
                            Background::Color(a.rail_color_1),
                            Background::Color(a.rail_color_2),
                        ),
                        width: a.rail_width,
                        border: iced::Border {
                            radius: a.rail_border_radius,
                            ..Default::default()
                        },
                    },
                    handle: slider::Handle {
                        shape: handle_shape,
                        background: a.handle_background,
                        border_width: a.handle_border.width,
                        border_color: a.handle_border.color,
                    },
                }
            });
        }

        // Progress bar with themed style
        let mut prog = progress_bar(0.0..=100.0, self.slider_value);
        if let Some(s) = &self.styles.progress_bar_style {
            let a = *s.appearance();
            prog = prog.style(move |_theme| progress_bar::Style {
                background: a.background,
                bar: a.bar,
                border: a.border,
            });
        }

        // Radio buttons
        let options = ["Option A", "Option B", "Option C"];
        let radios: Vec<Element<'_, Message>> = options
            .iter()
            .map(|&opt| {
                radio(opt, opt, self.selected_option, Message::RadioSelected).into()
            })
            .collect();

        let slider_label = text(format!("Slider: {:.0}", self.slider_value));

        let content = column![
            heading,
            input,
            row![btn, check].spacing(10),
            tog,
            slider_label,
            sld,
            text("Progress:"),
            prog,
            text("Radio:"),
            row(radios).spacing(10),
        ]
        .spacing(20)
        .padding(40)
        .max_width(600);

        let mut ct = container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        if let Some(s) = &self.styles.container_style {
            let a = *s.appearance();
            ct = ct.style(move |_theme| container::Style {
                text_color: a.text_color,
                background: a.background,
                border: a.border,
                shadow: a.shadow,
                snap: false,
            });
        }

        ct.into()
    }
}

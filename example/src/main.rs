use iced::widget::{
    button, checkbox, column, container, progress_bar, radio, row, slider, text, text_input,
    toggler,
};
use iced::{Element, Length, Theme};
use iced_themer::style::{
    ButtonStyle, CheckboxStyle, ContainerStyle, ProgressBarStyle, RadioStyle, SliderStyle,
    TextInputStyle, TogglerStyle,
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
        radio_style: config.radio().cloned(),
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
    radio_style: Option<RadioStyle>,
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

        // Text input
        let mut input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged);
        if let Some(s) = &self.styles.text_input_style {
            input = input.style(s.style_fn());
        }

        // Button
        let mut btn = button("Clear").on_press(Message::ButtonPressed);
        if let Some(s) = &self.styles.button_style {
            btn = btn.style(s.style_fn());
        }

        // Checkbox
        let mut check = checkbox(self.is_checked)
            .label("Enable feature")
            .on_toggle(Message::CheckboxToggled);
        if let Some(s) = &self.styles.checkbox_style {
            check = check.style(s.style_fn());
        }

        // Toggler
        let mut tog = toggler(self.is_toggled)
            .label("Dark mode")
            .on_toggle(Message::TogglerToggled);
        if let Some(s) = &self.styles.toggler_style {
            tog = tog.style(s.style_fn());
        }

        // Slider
        let mut sld = slider(0.0..=100.0, self.slider_value, Message::SliderChanged);
        if let Some(s) = &self.styles.slider_style {
            sld = sld.style(s.style_fn());
        }

        // Progress bar
        let mut prog = progress_bar(0.0..=100.0, self.slider_value);
        if let Some(s) = &self.styles.progress_bar_style {
            prog = prog.style(s.style_fn());
        }

        // Radio buttons
        let options = ["Option A", "Option B", "Option C"];
        let radios: Vec<Element<'_, Message>> = options
            .iter()
            .map(|&opt| {
                let mut r = radio(opt, opt, self.selected_option, Message::RadioSelected);
                if let Some(s) = &self.styles.radio_style {
                    r = r.style(s.style_fn());
                }
                r.into()
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
            ct = ct.style(s.style_fn());
        }

        ct.into()
    }
}

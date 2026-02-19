use std::sync::Arc;

use iced::widget::{
    button, checkbox, column, container, progress_bar, radio, row, slider, text, text_input,
    toggler,
};
use iced::{Element, Length};
use iced_themer::{ThemeConfig, Themed};

fn main() -> iced::Result {
    let light = Arc::new(
        ThemeConfig::from_file("example/light.toml").expect("failed to load light.toml"),
    );
    let dark = Arc::new(
        ThemeConfig::from_file("example/dark.toml").expect("failed to load dark.toml"),
    );

    let font = dark.font();

    let (boot_light, boot_dark) = (Arc::clone(&light), Arc::clone(&dark));
    let app = iced::application(
        move || App::new(Arc::clone(&boot_light), Arc::clone(&boot_dark)),
        App::update,
        App::view,
    )
    .title("iced-themer Demo")
    .theme(|state: &App| state.active_config().theme());

    match font {
        Some(f) => app.default_font(f).run(),
        None => app.run(),
    }
}

struct App {
    input_value: String,
    is_checked: bool,
    is_dark: bool,
    slider_value: f32,
    selected_option: Option<&'static str>,
    light: Arc<ThemeConfig>,
    dark: Arc<ThemeConfig>,
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
    fn new(light: Arc<ThemeConfig>, dark: Arc<ThemeConfig>) -> Self {
        Self {
            input_value: String::new(),
            is_checked: false,
            is_dark: true,
            slider_value: 50.0,
            selected_option: None,
            light,
            dark,
        }
    }

    fn active_config(&self) -> &ThemeConfig {
        if self.is_dark { &self.dark } else { &self.light }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => self.input_value.clear(),
            Message::CheckboxToggled(value) => self.is_checked = value,
            Message::TogglerToggled(value) => self.is_dark = value,
            Message::SliderChanged(value) => self.slider_value = value,
            Message::RadioSelected(value) => self.selected_option = Some(value),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let cfg = self.active_config();

        let heading = text("iced-themer Demo").size(28);

        let input = text_input("Type something...", &self.input_value)
            .on_input(Message::InputChanged)
            .themed(cfg.text_input());

        let btn = button("Clear")
            .on_press(Message::ButtonPressed)
            .themed(cfg.button());

        let check = checkbox(self.is_checked)
            .label("Enable feature")
            .on_toggle(Message::CheckboxToggled)
            .themed(cfg.checkbox());

        let tog = toggler(self.is_dark)
            .label("Dark mode")
            .on_toggle(Message::TogglerToggled)
            .themed(cfg.toggler());

        let sld = slider(0.0..=100.0, self.slider_value, Message::SliderChanged)
            .themed(cfg.slider());

        let prog = progress_bar(0.0..=100.0, self.slider_value)
            .themed(cfg.progress_bar());

        let options = ["Option A", "Option B", "Option C"];
        let radios: Vec<Element<'_, Message>> = options
            .iter()
            .map(|&opt| {
                radio(opt, opt, self.selected_option, Message::RadioSelected)
                    .themed(cfg.radio())
                    .into()
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

        container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .themed(cfg.container())
            .into()
    }
}

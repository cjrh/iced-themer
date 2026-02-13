# iced-themer

> **Alpha software** — API will change. Not ready for production use.

Parse TOML theme files into iced's native `Theme` type at runtime, so you can tweak colors, fonts, and per-widget styles without recompiling.

## Add to your project

```toml
[dependencies]
iced-themer = { git = "https://github.com/cjrh/iced-themer" }
iced = "0.14"
```

## Theme file

```toml
name = "Ocean Breeze"

[palette]
background = "#1B2838"
text = "#C7D5E0"
primary = "#66C0F4"
success = "#4CAF50"
warning = "#FFC107"
danger = "#F44336"

[font]
family = "Arial"
weight = "normal"

[button]
background = "#66C0F4"
text-color = "#FFFFFF"
border-radius = 4.0

[button.hovered]
background = "#77D0FF"

[button.pressed]
background = "#5590C0"

[button.disabled]
background = "#445566"
text-color = "#888888"

# Backgrounds can also be linear gradients (up to 8 color stops):
[progress-bar]
border-radius = 4.0

[progress-bar.bar]
angle = 90
stops = [
  { offset = 0.0, color = "#66C0F4" },
  { offset = 1.0, color = "#4CAF50" },
]

# For more see the `theme.toml` in the example/ directory.
```

Every widget section is optional — omit it and the iced default applies. Status sub-tables (`hovered`, `pressed`, etc.) inherit from the base and only override what they specify.

## Usage

```rust
use iced::widget::button;
use iced::{Element, Theme};
use iced_themer::ThemeConfig;

fn main() -> iced::Result {
    let config = ThemeConfig::from_file("theme.toml")
        .expect("failed to load theme");

    let theme = config.theme();   // cheap Arc clone
    let font = config.font();     // Option<Font>
    let btn = config.button().cloned();

    let app = iced::application(|| App { btn }, App::update, App::view)
        .title("My App")
        .theme(move |_: &App| -> Theme { theme.clone() });

    match font {
        Some(f) => app.default_font(f).run(),
        None => app.run(),
    }
}

struct App {
    btn: Option<iced_themer::style::ButtonStyle>,
}

impl App {
    fn update(&mut self, _msg: ()) {}

    fn view(&self) -> Element<'_, ()> {
        let mut b = button("Click me").on_press(());
        if let Some(s) = &self.btn {
            b = b.style(s.style_fn());
        }
        b.into()
    }
}
```

Each widget style type provides a `style_fn()` method that returns a closure ready for `.style()`. No manual status matching needed.

## Supported widgets

- button
- checkbox
- container
- progress-bar
- radio
- slider
- text-input
- toggler

## License

MIT

## See Also

- [marcel](https://github.com/micro-rust/marcel)

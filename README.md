# iced-themer

> **Alpha software**: API will change. Not ready for production use.

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

# Variables: define once, reference with $name anywhere in the file.
# Derived colors can be computed with transformation functions:
#   darken, lighten, saturate, desaturate, tint, shade, greyscale, spin, mix
[variables]
bg        = "#1B2838"
bg-raised = "#2A3F5F"
text      = "#C7D5E0"
primary   = "#66C0F4"
primary-h = "lighten($primary, 8%)"    # lighter on hover
primary-d = "darken($primary, 15%)"    # darker on press
muted     = "desaturate($text, 60%)"   # derived from $text

[palette]
background = "$bg"
text       = "$text"
primary    = "$primary"
success    = "#4CAF50"
warning    = "#FFC107"
danger     = "#F44336"

[font]
family = "Arial"
weight = "normal"

[button]
background    = "$primary"
text-color    = "#FFFFFF"
border-radius = 4.0

[button.hovered]
background = "$primary-h"

[button.pressed]
background = "$primary-d"

[button.disabled]
background = "#445566"
text-color = "$muted"

# Backgrounds can also be linear gradients (up to 8 color stops):
[progress-bar.bar]
angle = 90
stops = [
  { offset = 0.0, color = "$primary" },
  { offset = 1.0, color = "#4CAF50"  },
]

# For more see `dark.toml` / `light.toml` in the example/ directory.
```

Every widget section is optional.
Omit it and the iced default applies.
Status sub-tables (`hovered`, `pressed`, etc.) inherit from the base and only override what they specify.

## Usage

```rust
use std::sync::Arc;
use iced::{Element, Theme};
use iced_themer::{ThemeConfig, Themed};

fn main() -> iced::Result {
    let config = Arc::new(
        ThemeConfig::from_file("theme.toml").expect("failed to load theme"),
    );

    let theme = config.theme();  // cheap Arc clone
    let font  = config.font();   // Option<Font>

    let boot_config = Arc::clone(&config);
    let app = iced::application(move || App::new(Arc::clone(&boot_config)), App::update, App::view)
        .title("My App")
        .theme(move |_: &App| -> Theme { theme.clone() });

    match font {
        Some(f) => app.default_font(f).run(),
        None    => app.run(),
    }
}

struct App {
    value: f32,
    config: Arc<ThemeConfig>,
}

impl App {
    fn new(config: Arc<ThemeConfig>) -> Self {
        Self { value: 0.5, config }
    }

    fn update(&mut self, v: f32) { self.value = v; }

    fn view(&self) -> Element<'_, f32> {
        use iced::widget::{button, slider};

        // Import Themed once: all 8 supported widgets gain .themed()
        slider(0.0..=1.0, self.value, |v| v)
            .themed(self.config.slider())
            .into()
    }
}
```

`.themed(None)` returns the widget unchanged, so missing TOML sections silently fall back to the palette defaults.

## Variables and color functions

Define named values in `[variables]` and reference them with `$name` anywhere a color is expected.
Variables can themselves use color transformation expressions:

| Function | Parameters | Effect |
|---|---|---|
| `darken(color, pct%)` | percent | decrease lightness |
| `lighten(color, pct%)` | percent | increase lightness |
| `saturate(color, pct%)` | percent | increase saturation |
| `desaturate(color, pct%)` | percent | decrease saturation |
| `tint(color, pct%)` | percent | mix towards white |
| `shade(color, pct%)` | percent | mix towards black |
| `greyscale(color)` | - | remove all saturation |
| `spin(color, deg)` | degrees | rotate hue |
| `mix(color, color, pct%)` | percent | blend two colors |

## Supported widgets

| TOML section     | Status sub-tables                             |
|------------------|-----------------------------------------------|
| `[button]`       | `hovered`, `pressed`, `disabled`              |
| `[checkbox]`     | `checked`, `hovered`, `hovered-checked`, `disabled`, `disabled-checked` |
| `[container]`    | -                                             |
| `[progress-bar]` | -                                             |
| `[radio]`        | `selected`, `hovered`, `hovered-selected`     |
| `[slider]`       | `hovered`, `dragged`                          |
| `[text-input]`   | `focused`, `disabled`                         |
| `[toggler]`      | `toggled`, `hovered`, `hovered-toggled`, `disabled`, `disabled-toggled` |

## Switching themes at runtime

Store multiple `ThemeConfig`s in your app state and read the active one from the `.theme()` closure each frame. Because the closure runs on every render, the theme switches instantly.

```rust
struct App {
    is_dark: bool,
    light: Arc<ThemeConfig>,
    dark: Arc<ThemeConfig>,
}

impl App {
    fn active_config(&self) -> &ThemeConfig {
        if self.is_dark { &self.dark } else { &self.light }
    }
}
```

Wire the `.theme()` closure to read from state instead of capturing a fixed clone:

```rust
// Before (stale: won't reflect runtime changes):
let theme = config.theme();
.theme(move |_: &App| -> Theme { theme.clone() })

// After (live: reads from state each frame):
.theme(|state: &App| state.active_config().theme())
```

The `Arc` wrapper satisfies iced's `Fn` requirement on the boot closure.
Both configs can be cloned cheaply into the initial state:

```rust
let light = Arc::new(ThemeConfig::from_file("light.toml").expect("…"));
let dark  = Arc::new(ThemeConfig::from_file("dark.toml").expect("…"));

let (boot_light, boot_dark) = (Arc::clone(&light), Arc::clone(&dark));
iced::application(
    move || App::new(Arc::clone(&boot_light), Arc::clone(&boot_dark)),
    App::update,
    App::view,
)
.theme(|state: &App| state.active_config().theme())
```

See `example/` for a complete demo with a toggler that switches between `light.toml` and `dark.toml`.

## Embedding a theme in your binary

To avoid shipping a separate `.toml` file alongside your binary, bake the theme in at compile time:

```rust
let config: ThemeConfig = include_str!("../theme.toml").parse().unwrap();
```

The `FromStr` impl on `ThemeConfig` accepts the same TOML content as `from_file`.

## License

MIT

## See Also

- [marcel](https://github.com/micro-rust/marcel)

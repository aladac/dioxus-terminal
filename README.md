# dioxus-terminal

Terminal emulator widget for [Dioxus](https://dioxuslabs.com/) desktop applications.

Built on [alacritty_terminal](https://crates.io/crates/alacritty_terminal) for terminal emulation and [portable-pty](https://crates.io/crates/portable-pty) for cross-platform PTY support.

## Features

- Full terminal emulation (VT100/xterm compatible)
- ANSI color support (16, 256, and true color)
- Keyboard and mouse input
- Scrollback buffer
- Customizable themes

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-terminal = "0.1"
```

## Usage

```rust
use dioxus::prelude::*;
use dioxus_terminal::Terminal;

fn app() -> Element {
    rsx! {
        Terminal {
            command: "bash",
            rows: 24,
            cols: 80,
        }
    }
}
```

## Customization

```rust
use dioxus_terminal::{Terminal, Color};

rsx! {
    Terminal {
        command: "/bin/zsh",
        args: vec!["-l".to_string()],
        rows: 30,
        cols: 120,
        font_size: 16,
        font_family: "JetBrains Mono",
        background: Color::new(30, 30, 30),
        foreground: Color::new(220, 220, 220),
    }
}
```

## License

MIT

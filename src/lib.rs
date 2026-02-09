//! # dioxus-terminal
//!
//! Terminal emulator widget for Dioxus desktop applications.
//!
//! Built on top of `alacritty_terminal` for terminal emulation and
//! `portable-pty` for cross-platform PTY support.
//!
//! ## Features
//!
//! - Full terminal emulation (VT100/xterm compatible)
//! - ANSI color support (16, 256, and true color)
//! - Keyboard and mouse input
//! - Scrollback buffer
//! - Copy/paste support
//! - Customizable themes
//!
//! ## Example
//!
//! ```ignore
//! use dioxus::prelude::*;
//! use dioxus_terminal::Terminal;
//!
//! fn app() -> Element {
//!     rsx! {
//!         Terminal {
//!             command: "bash",
//!             rows: 24,
//!             cols: 80,
//!         }
//!     }
//! }
//! ```

mod error;
mod pty;
mod term;
mod widget;

pub use error::Error;
pub use pty::Pty;
pub use term::{Cell, Color, Grid, Style};
pub use widget::{Terminal, TerminalProps};

/// Result type for dioxus-terminal operations
pub type Result<T> = std::result::Result<T, Error>;

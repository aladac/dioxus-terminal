//! Dioxus terminal widget component

use dioxus::prelude::*;

use crate::term::{Color, Grid};

/// Props for the Terminal component
#[derive(Props, Clone, PartialEq)]
pub struct TerminalProps {
    /// Command to run (default: user's shell)
    #[props(default = default_shell())]
    pub command: String,

    /// Command arguments
    #[props(default)]
    pub args: Vec<String>,

    /// Number of rows (default: 24)
    #[props(default = 24)]
    pub rows: u16,

    /// Number of columns (default: 80)
    #[props(default = 80)]
    pub cols: u16,

    /// Font size in pixels (default: 14)
    #[props(default = 14)]
    pub font_size: u16,

    /// Font family (default: monospace)
    #[props(default = "monospace".to_string())]
    pub font_family: String,

    /// Background color
    #[props(default = Color::default_bg())]
    pub background: Color,

    /// Foreground color
    #[props(default = Color::default_fg())]
    pub foreground: Color,

    /// CSS class for the container
    #[props(default)]
    pub class: String,
}

fn default_shell() -> String {
    std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
}

/// Terminal emulator widget for Dioxus
///
/// # Example
///
/// ```ignore
/// use dioxus::prelude::*;
/// use dioxus_terminal::Terminal;
///
/// fn app() -> Element {
///     rsx! {
///         Terminal {
///             command: "bash",
///             rows: 24,
///             cols: 80,
///         }
///     }
/// }
/// ```
#[component]
pub fn Terminal(props: TerminalProps) -> Element {
    let grid = use_signal(|| Grid::new(props.rows as usize, props.cols as usize));

    let container_style = format!(
        "background-color: {}; color: {}; font-family: {}; font-size: {}px;",
        props.background.to_css(),
        props.foreground.to_css(),
        props.font_family,
        props.font_size
    );

    let container_class = format!(
        "terminal-container overflow-hidden select-none {}",
        props.class
    );

    rsx! {
        div {
            class: "{container_class}",
            style: "{container_style}",
            tabindex: "0",
            onkeydown: move |evt| {
                // TODO: Send keypress to PTY
                let _key = evt.key();
            },

            // Render grid
            div { class: "terminal-grid whitespace-pre",
                for (row_idx, row) in grid.read().iter_rows().enumerate() {
                    div { class: "terminal-row", key: "{row_idx}",
                        for (col_idx, cell) in row.iter().enumerate() {
                            span {
                                key: "{col_idx}",
                                class: "{cell.style.to_css_classes()}",
                                style: "color: {cell.fg.to_css()}; background-color: {cell.bg.to_css()};",
                                "{cell.c}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_shell() {
        let shell = default_shell();
        assert!(!shell.is_empty());
    }

    #[test]
    fn test_terminal_props_defaults() {
        let props = TerminalProps {
            command: "bash".to_string(),
            args: vec![],
            rows: 24,
            cols: 80,
            font_size: 14,
            font_family: "monospace".to_string(),
            background: Color::default_bg(),
            foreground: Color::default_fg(),
            class: String::new(),
        };

        assert_eq!(props.rows, 24);
        assert_eq!(props.cols, 80);
        assert_eq!(props.font_size, 14);
    }

    #[test]
    fn test_color_default_bg() {
        let bg = Color::default_bg();
        assert_eq!(bg.r, 0);
        assert_eq!(bg.g, 0);
        assert_eq!(bg.b, 0);
    }

    #[test]
    fn test_color_default_fg() {
        let fg = Color::default_fg();
        assert_eq!(fg.r, 204);
        assert_eq!(fg.g, 204);
        assert_eq!(fg.b, 204);
    }
}

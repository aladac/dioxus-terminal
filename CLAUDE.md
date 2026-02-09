# dioxus-terminal

Terminal emulator widget for Dioxus desktop applications.

## Architecture

```
src/
├── lib.rs      # Public API exports
├── error.rs    # Error types (thiserror)
├── pty.rs      # PTY management (portable-pty)
├── term.rs     # Terminal types: Grid, Cell, Color, Style
└── widget.rs   # Dioxus Terminal component
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `alacritty_terminal` | VT100/xterm terminal emulation |
| `portable-pty` | Cross-platform PTY (from wezterm) |
| `dioxus` | GUI framework |
| `tokio` | Async runtime for PTY I/O |

## Key Types

- `Pty` - PTY handle, spawns commands, read/write I/O
- `Grid` - 2D array of cells
- `Cell` - Character + foreground + background + style
- `Color` - RGB color with CSS/hex conversion
- `Style` - Bold, italic, underline, etc.
- `Terminal` - Dioxus component

## Testing

```bash
cargo test           # Run tests
cargo tarpaulin      # Coverage
```

## Commands

- `/check` - Full validation (fmt, clippy, test, coverage)
- `/coverage` - Coverage report
- `/lint` - Format and clippy

## TODO

- [ ] Integrate alacritty_terminal Parser for ANSI handling
- [ ] Connect PTY output to Grid
- [ ] Handle keyboard input properly
- [ ] Mouse support
- [ ] Scrollback buffer
- [ ] Selection and copy/paste
- [ ] Themes/color schemes

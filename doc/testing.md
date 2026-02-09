# Testing

## Coverage Requirement

Maintain high coverage for library code.

## Test Organization

### Unit Tests (Inline)

For private functions and module internals:

```rust
fn helper(x: u32) -> u32 { x * 2 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper(5), 10);
    }
}
```

### Integration Tests

For public API, in `tests/` directory:

```rust
// tests/test_api.rs
use dioxus_terminal::{Terminal, Pty};

#[test]
fn test_pty_spawn() {
    let pty = Pty::spawn("echo", &["hello"], 24, 80);
    assert!(pty.is_ok());
}
```

### Component Tests with SSR

Use dioxus-ssr for component testing:

```rust
use dioxus::prelude::*;
use dioxus_terminal::Terminal;

#[test]
fn test_terminal_renders() {
    let html = dioxus_ssr::render_element(rsx! {
        Terminal { command: "bash", rows: 24, cols: 80 }
    });
    assert!(html.contains("terminal-container"));
}
```

## Testing PTY

PTY tests spawn real processes. Keep them fast:

```rust
#[test]
fn test_echo_command() {
    let pty = Pty::spawn("echo", &["test"], 24, 80).unwrap();
    // Echo exits immediately, no need to wait
    assert_eq!(pty.size(), (24, 80));
}
```

## Running Tests

```bash
cargo test                    # All tests
cargo test pty::tests         # Specific module
cargo test --doc              # Doc tests only
cargo tarpaulin --skip-clean  # Coverage
```

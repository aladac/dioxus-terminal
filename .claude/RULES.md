# Project Rules

Read this BEFORE writing code. Not after.

## Naming

**NO redundant prefixes:**
- `config::Config` → `Config`
- `module::ModuleThing` → `module::Thing`

## Imports

Order: std → external crates → crate → super/self

```rust
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

use crate::Config;

use super::types::Request;
```

## Code Limits

| Metric | Limit |
|--------|-------|
| Line width | 120 |
| Function body | 50 lines |
| Arguments | 5 max |
| File length | ~300 lines |

## Forbidden

- `.unwrap()` in lib code (ok in tests)
- `panic!()` for recoverable errors
- `use module::*`
- `dbg!()` or `todo!()` in commits
- Clippy warnings
- Unformatted code

## Testing

**Write tests alongside code, not after.**

Every public function needs:
- Unit test for happy path
- Unit test for error paths
- Doc test if it's part of public API

```rust
/// Parses a model name from a path.
///
/// # Examples
///
/// ```
/// use mylib::parse_name;
/// assert_eq!(parse_name("model_v1.safetensors"), "model_v1");
/// ```
pub fn parse_name(path: &str) -> &str {
    // implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name_simple() {
        assert_eq!(parse_name("model.safetensors"), "model");
    }

    #[test]
    fn test_parse_name_with_version() {
        assert_eq!(parse_name("model_v2.safetensors"), "model_v2");
    }
}
```

See `doc/testing.md` for patterns.

## Before Finishing

```bash
cargo fmt
cargo clippy  # must be zero warnings
cargo test
cargo tarpaulin --skip-clean  # maintain 100%
```

Or run `/check` command.

//! Error types for dioxus-terminal

use thiserror::Error;

/// Errors that can occur in dioxus-terminal
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to create PTY
    #[error("failed to create PTY: {0}")]
    PtyCreation(String),

    /// Failed to spawn command
    #[error("failed to spawn command: {0}")]
    SpawnCommand(String),

    /// PTY I/O error
    #[error("PTY I/O error: {0}")]
    PtyIo(#[from] std::io::Error),

    /// Terminal size error
    #[error("invalid terminal size: {rows}x{cols}")]
    InvalidSize { rows: u16, cols: u16 },

    /// Command not found
    #[error("command not found: {0}")]
    CommandNotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_pty_creation() {
        let err = Error::PtyCreation("test error".to_string());
        assert_eq!(err.to_string(), "failed to create PTY: test error");
    }

    #[test]
    fn error_display_spawn_command() {
        let err = Error::SpawnCommand("permission denied".to_string());
        assert_eq!(
            err.to_string(),
            "failed to spawn command: permission denied"
        );
    }

    #[test]
    fn error_display_invalid_size() {
        let err = Error::InvalidSize { rows: 0, cols: 80 };
        assert_eq!(err.to_string(), "invalid terminal size: 0x80");
    }

    #[test]
    fn error_display_command_not_found() {
        let err = Error::CommandNotFound("zsh".to_string());
        assert_eq!(err.to_string(), "command not found: zsh");
    }

    #[test]
    fn error_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(err.to_string().contains("file not found"));
    }
}

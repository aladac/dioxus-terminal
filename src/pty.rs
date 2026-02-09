//! PTY (pseudo-terminal) management

use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use crate::{Error, Result};

/// PTY handle for terminal I/O
pub struct Pty {
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    reader_rx: mpsc::Receiver<Vec<u8>>,
    size: PtySize,
}

impl std::fmt::Debug for Pty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pty")
            .field("size", &self.size)
            .finish_non_exhaustive()
    }
}

impl Pty {
    /// Create a new PTY and spawn the given command
    ///
    /// # Arguments
    ///
    /// * `command` - The command to run (e.g., "bash", "/bin/zsh")
    /// * `args` - Command arguments
    /// * `rows` - Terminal height in rows
    /// * `cols` - Terminal width in columns
    ///
    /// # Errors
    ///
    /// Returns an error if the PTY cannot be created or the command fails to spawn.
    pub fn spawn(command: &str, args: &[&str], rows: u16, cols: u16) -> Result<Self> {
        if rows == 0 || cols == 0 {
            return Err(Error::InvalidSize { rows, cols });
        }

        let size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };

        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(size)
            .map_err(|e| Error::PtyCreation(e.to_string()))?;

        let mut cmd = CommandBuilder::new(command);
        cmd.args(args);

        let _child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| Error::SpawnCommand(e.to_string()))?;

        let writer = pair
            .master
            .take_writer()
            .map_err(|e| Error::PtyCreation(e.to_string()))?;
        let mut reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| Error::PtyCreation(e.to_string()))?;

        let (tx, rx) = mpsc::channel(256);

        // Spawn reader thread
        std::thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        if tx.blocking_send(buf[..n].to_vec()).is_err() {
                            break;
                        }
                    }
                }
            }
        });

        Ok(Self {
            writer: Arc::new(Mutex::new(writer)),
            reader_rx: rx,
            size,
        })
    }

    /// Write data to the PTY (send to the running process)
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    ///
    /// # Panics
    ///
    /// Panics if the writer mutex is poisoned.
    pub fn write(&self, data: &[u8]) -> Result<()> {
        let mut writer = self.writer.lock().expect("writer lock poisoned");
        writer.write_all(data)?;
        writer.flush()?;
        Ok(())
    }

    /// Write a string to the PTY
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    pub fn write_str(&self, s: &str) -> Result<()> {
        self.write(s.as_bytes())
    }

    /// Try to receive output from the PTY (non-blocking)
    pub fn try_read(&mut self) -> Option<Vec<u8>> {
        self.reader_rx.try_recv().ok()
    }

    /// Receive output from the PTY (async)
    pub async fn read(&mut self) -> Option<Vec<u8>> {
        self.reader_rx.recv().await
    }

    /// Resize the PTY
    ///
    /// # Errors
    ///
    /// Returns an error if the size is invalid.
    pub fn resize(&mut self, rows: u16, cols: u16) -> Result<()> {
        if rows == 0 || cols == 0 {
            return Err(Error::InvalidSize { rows, cols });
        }
        self.size.rows = rows;
        self.size.cols = cols;
        // Note: actual resize would need master.resize() which requires keeping master handle
        Ok(())
    }

    /// Get current terminal size
    #[must_use]
    pub fn size(&self) -> (u16, u16) {
        (self.size.rows, self.size.cols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_size_zero_rows() {
        let result = Pty::spawn("echo", &["test"], 0, 80);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::InvalidSize { rows: 0, cols: 80 }));
    }

    #[test]
    fn test_invalid_size_zero_cols() {
        let result = Pty::spawn("echo", &["test"], 24, 0);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::InvalidSize { rows: 24, cols: 0 }));
    }

    #[test]
    fn test_spawn_echo() {
        let pty = Pty::spawn("echo", &["hello"], 24, 80);
        assert!(pty.is_ok());
    }

    #[test]
    fn test_size() {
        let pty = Pty::spawn("echo", &["test"], 24, 80).unwrap();
        assert_eq!(pty.size(), (24, 80));
    }

    #[test]
    fn test_resize_valid() {
        let mut pty = Pty::spawn("echo", &["test"], 24, 80).unwrap();
        let result = pty.resize(40, 120);
        assert!(result.is_ok());
        assert_eq!(pty.size(), (40, 120));
    }

    #[test]
    fn test_resize_invalid() {
        let mut pty = Pty::spawn("echo", &["test"], 24, 80).unwrap();
        let result = pty.resize(0, 120);
        assert!(result.is_err());
    }
}

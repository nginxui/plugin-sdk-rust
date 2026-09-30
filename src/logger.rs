//! The plugin logger.
//!
//! Lines go to stderr, since stdout belongs to the protocol, and are mirrored
//! to the host log once the connection is up.

use std::fmt;
use std::io::Write;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};

use serde_json::Value;

use crate::host::current_host;
use crate::protocol::Settings;

/// A log severity understood by `host.log`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    /// Diagnostic detail.
    Debug,
    /// Normal operation.
    Info,
    /// Something is off but the plugin carries on.
    Warn,
    /// A failure.
    Error,
}

impl Level {
    /// The name the protocol uses, `debug`, `info`, `warn` or `error`.
    pub fn as_str(self) -> &'static str {
        match self {
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warn => "warn",
            Level::Error => "error",
        }
    }

    fn rank(self) -> u8 {
        self as u8
    }

    fn from_rank(rank: u8) -> Level {
        match rank {
            0 => Level::Debug,
            2 => Level::Warn,
            3 => Level::Error,
            _ => Level::Info,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Writes human readable lines to stderr and mirrors them to the host once
/// the connection is up. stdout is reserved for the protocol, so no log line
/// is ever written there.
pub struct Logger {
    out: Mutex<Box<dyn Write + Send>>,
    min: AtomicU8,
    host_min: AtomicU8,
}

impl fmt::Debug for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Logger")
            .field("min", &self.min_level())
            .field("host_min", &self.host_min_level())
            .finish_non_exhaustive()
    }
}

impl Logger {
    /// Builds a logger writing to `out`. It logs everything and forwards
    /// `info` and above to the host.
    pub fn new(out: impl Write + Send + 'static) -> Logger {
        Logger {
            out: Mutex::new(Box::new(out)),
            min: AtomicU8::new(Level::Debug.rank()),
            host_min: AtomicU8::new(Level::Info.rank()),
        }
    }

    /// Replaces the fallback writer, mainly for tests.
    pub fn set_output(&self, out: impl Write + Send + 'static) {
        *self.out.lock().expect("logger lock") = Box::new(out);
    }

    /// Sets the lowest level written at all.
    pub fn set_min_level(&self, level: Level) {
        self.min.store(level.rank(), Ordering::Relaxed);
    }

    /// The lowest level written at all.
    pub fn min_level(&self) -> Level {
        Level::from_rank(self.min.load(Ordering::Relaxed))
    }

    /// Sets the lowest level forwarded to `host.log`. Lines below it, and
    /// lines emitted before the host is ready, only reach stderr.
    pub fn set_host_min_level(&self, level: Level) {
        self.host_min.store(level.rank(), Ordering::Relaxed);
    }

    /// The lowest level forwarded to `host.log`.
    pub fn host_min_level(&self) -> Level {
        Level::from_rank(self.host_min.load(Ordering::Relaxed))
    }

    /// Emits one line at the given level, with structured fields.
    pub fn log(&self, level: Level, msg: &str, fields: Option<&Settings>) {
        if level < self.min_level() {
            return;
        }

        if level >= self.host_min_level() {
            if let Some(host) = current_host() {
                if host.ready() {
                    let empty = Settings::new();
                    if host.log(level, msg, fields.unwrap_or(&empty)).is_ok() {
                        return;
                    }
                    // Fall through to stderr when the host call could not be
                    // delivered.
                }
            }
        }

        self.write_stderr(level, msg, fields);
    }

    fn write_stderr(&self, level: Level, msg: &str, fields: Option<&Settings>) {
        let mut line = format!("[{}] {msg}", level.as_str().to_uppercase());
        if let Some(fields) = fields {
            for (k, v) in fields {
                line.push(' ');
                line.push_str(k);
                line.push('=');
                match v {
                    Value::String(s) => line.push_str(s),
                    other => line.push_str(&other.to_string()),
                }
            }
        }
        line.push('\n');

        let mut out = self.out.lock().expect("logger lock");
        let _ = out.write_all(line.as_bytes());
        let _ = out.flush();
    }

    /// Logs at debug level.
    pub fn debug(&self, msg: impl AsRef<str>) {
        self.log(Level::Debug, msg.as_ref(), None);
    }

    /// Logs at info level.
    pub fn info(&self, msg: impl AsRef<str>) {
        self.log(Level::Info, msg.as_ref(), None);
    }

    /// Logs at warn level.
    pub fn warn(&self, msg: impl AsRef<str>) {
        self.log(Level::Warn, msg.as_ref(), None);
    }

    /// Logs at error level.
    pub fn error(&self, msg: impl AsRef<str>) {
        self.log(Level::Error, msg.as_ref(), None);
    }
}

/// Returns the process wide logger. It writes to stderr.
pub fn logger() -> &'static Logger {
    static LOGGER: OnceLock<Logger> = OnceLock::new();
    LOGGER.get_or_init(|| Logger::new(std::io::stderr()))
}

/// Logs to the process wide logger at debug level.
pub fn debug(msg: impl AsRef<str>) {
    logger().debug(msg);
}

/// Logs to the process wide logger at info level.
pub fn info(msg: impl AsRef<str>) {
    logger().info(msg);
}

/// Logs to the process wide logger at warn level.
pub fn warn(msg: impl AsRef<str>) {
    logger().warn(msg);
}

/// Logs to the process wide logger at error level.
pub fn error(msg: impl AsRef<str>) {
    logger().error(msg);
}

/// Logs a message with structured fields on the process wide logger.
pub fn log_fields(level: Level, msg: &str, fields: &Settings) {
    logger().log(level, msg, Some(fields));
}

/// Logs a formatted message at debug level on the process wide logger.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logger::debug(::std::format!($($arg)*))
    };
}

/// Logs a formatted message at info level on the process wide logger.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::info(::std::format!($($arg)*))
    };
}

/// Logs a formatted message at warn level on the process wide logger.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::warn(::std::format!($($arg)*))
    };
}

/// Logs a formatted message at error level on the process wide logger.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::error(::std::format!($($arg)*))
    };
}

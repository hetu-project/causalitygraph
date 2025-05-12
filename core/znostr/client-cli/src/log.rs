use std::fmt;
use clap::ValueEnum;

/// Enumerator used for --log-level option
#[derive(Clone, Debug, Copy, PartialEq, Default, ValueEnum)]
pub enum LogLevel {
    /// None: not set, default.
    #[default]
    None,
    /// Error: Indicates to print only errors
    Error,
    /// Warn: Indicates to print warnings and errors
    Warn,
    /// Info: Indicates to to print info, warn and errors
    Info,
    /// Debug: Indicates to to print debug and the rest
    Debug,
    /// Trace: Indicates to to print everything
    Trace,
}

/// is_ functions for the enum
impl LogLevel {
    pub fn is_none(&self) -> bool {
        self == &Self::None
    }
    // pub fn is_error(&self) -> bool { self == &Self::Error }
}

/// Creates .to_string() for Listen for --listen option
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

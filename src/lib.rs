#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/pretty_flexible_env_logger/latest/")]

//! A logger configured via run-time value or one taken from an environment variable which
//! writes to standard error with nice colored output for log levels.
//!
//! ## Example
//!
//! ```
//! use log::{debug,info,warn,error};
//! use std::env;
//!
//! fn main() {
//!     let args: Vec<String> = env::args().collect();
//!     let default = "RUST_LOG".to_string();
//!     let level = args.get(1).unwrap_or(&default);
//!     if let Err(e) = pretty_flexible_env_logger::try_init_with(level) {
//!         eprintln!("Some custom msg {}", e);
//!         panic!("error!") // or whatever
//!     }
//!
//!     info!("info");
//!     warn!("warn");
//!     error!("error");
//!     debug!("debug");
//! }
//! ```
//!
//! ## Defaults
//!
//! The defaults can be setup by calling [init()][init] or [try_init()][try_init] at the start
//! of the program.
//!
//! ## Enable logging
//!
//! This crate uses [pretty_env_logger][] internally, so the same ways of enabling
//! logs through an environment variable are supported.
//!
//! [env_logger]: https://docs.rs/env_logger
//! [pretty_env_logger]: https://docs.rs/pretty_env_logger
//! [init]: [pretty_flexible_env_logger::init]
//! [try_init]: [pretty_flexible_env_logger::try_init]

#[doc(hidden)]
pub use pretty_env_logger;

#[doc(hidden)]
pub use pretty_env_logger::env_logger;

use log::SetLoggerError;
use pretty_env_logger::{formatted_builder, formatted_timed_builder};

/// Initializes default global logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// It defaults to using settings from `RUST_LOG` environment variable
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init() {
    init_with("RUST_LOG");
}

/// Initializes default global logger with timed entries.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// It defaults to using settings from `RUST_LOG` environment variable
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init_timed() {
    init_timed_with("RUST_LOG");
}

/// Tries to initialize default global logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init() -> Result<(), SetLoggerError> {
    try_init_with("RUST_LOG")
}

/// Tries to initialize default global logger with timed entries.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_timed() -> Result<(), SetLoggerError> {
    try_init_timed_with("RUST_LOG")
}

/// Initializes the global logger with a custom configuration.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Arguments
///
/// * `environment_or_inline_value` - A string slice that holds the name of environment variable, or
///    the directives string in the same form as the `RUST_LOG` environment variable.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init_with(environment_or_inline_value: &str) {
    try_init_with(environment_or_inline_value).unwrap();
}

/// Initializes the timed global logger with a custom configuration.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Arguments
///
/// * `environment_or_inline_value` - A string slice that holds the name of environment variable, or
///    the directives string in the same form as the `RUST_LOG` environment variable.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn init_timed_with(environment_or_inline_value: &str) {
    try_init_with(environment_or_inline_value).unwrap()
}

/// Tries to initialize the global logger with a custom configuration.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Arguments
///
/// * `environment_or_inline_value` - A string slice that holds the name of environment variable, or
///    the directives string in the same form as the `RUST_LOG` environment variable.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_with(environment_or_inline_value: &str) -> Result<(), SetLoggerError> {
    let value = match ::std::env::var(environment_or_inline_value) {
        Ok(s) => Some(s),
        Err(_) => Some(environment_or_inline_value.to_string()),
    };
    try_init_custom_string(value)
}

/// Tries to initialize the timed global logger with a custom configuration.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Arguments
///
/// * `environment_or_inline_value` - A string slice that holds the name of environment variable, or
///    the directives string in the same form as the `RUST_LOG` environment variable.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_timed_with(environment_or_inline_value: &str) -> Result<(), log::SetLoggerError> {
    let value = match ::std::env::var(environment_or_inline_value) {
        Ok(s) => Some(s),
        Err(_) => Some(environment_or_inline_value.to_string()),
    };
    try_init_timed_custom_string(value)
}

/// Tries to initialize the global logger with custom filtering directives.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Arguments
///
/// * `filters` - A directives `String` in the same form as the `RUST_LOG` environment variable.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_custom_string(filters: Option<String>) -> Result<(), SetLoggerError> {
    let mut builder = formatted_builder();

    if let Some(s) = filters {
        builder.parse_filters(&s);
    }

    builder.try_init()
}

/// Tries to initialize the timed global logger with custom filtering directives.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Arguments
///
/// * `filters` - A directives `String` in the same form as the `RUST_LOG` environment variable.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_timed_custom_string(filters: Option<String>) -> Result<(), SetLoggerError> {
    let mut builder = formatted_timed_builder();

    if let Some(s) = filters {
        builder.parse_filters(&s);
    }

    builder.try_init()
}

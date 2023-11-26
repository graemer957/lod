#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

#[cfg(target_os = "macos")]
mod app_state;
#[cfg(target_os = "macos")]
pub use app_state::{AppState, Mode};
mod config;
pub use config::Config;
#[cfg(target_os = "macos")]
mod menu_item;
pub mod program;

use program::{Program, ProgramImpl};
use std::error::Error;
use std::process::Command;

/// Make use of the `defaults` builtin macOS command line tool to get if the Dock is set to autohide
///
/// **NOTE**: I may look for a better way™ to do this in future - this is a quick hack project after all ;-)
///
/// # Errors
///
/// Could return a multitude of errors, say from `Program` or another reason as string slice
///
/// # Panics
///
/// If it finds an unexpected digit in the response from `defaults`
pub fn dock_autohide() -> Result<bool, Box<dyn Error>> {
    let mut defaults = Command::new("defaults");
    defaults.args(["read", "com.apple.dock", "autohide"]);
    let output = ProgramImpl::new(defaults, 0).execute()?;

    if output.stdout().len() != 2 {
        return Err("Got more chars from output than expected".into());
    }
    let digit = output
        .stdout()
        .first()
        .ok_or("Could not get first byte of output")?;

    match *digit {
        b'0' => Ok(false),
        b'1' => Ok(true),
        // Could have chosen to return `false` here, but would like to understand
        // a little more how this could happen during development
        _ => panic! {"Unexpected digit: {digit}"},
    }
}

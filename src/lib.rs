#[cfg(target_os = "macos")]
mod app_state;
#[cfg(target_os = "macos")]
pub use app_state::{AppState, Mode};
#[cfg(target_os = "macos")]
mod menu_item;

use std::error::Error;
use std::process::Command;

const NUMBER_ZERO_IN_ASCII: u8 = 48;
const NUMBER_ONE_IN_ASCII: u8 = 49;

/// Make use of the `defaults` builtin macOS command line tool to get if the Dock is set to autohide
///
/// **NOTE**: I may look for a better way™ to do this in future - this is a quick hack project after all ;-)
pub fn dock_autohide() -> Result<bool, Box<dyn Error>> {
    let output = Command::new("defaults")
        .args(["read", "com.apple.dock", "autohide"])
        .output()?;

    let status_code = output.status.code().ok_or("Could not get status code")?;
    if status_code != 0 {
        return Err("Got non-zero exit status for `defaults`".into());
    }
    if output.stdout.len() != 2 {
        return Err("Got more chars from output than expected".into());
    }
    let digit = output
        .stdout
        .first()
        .ok_or("Could not get first byte of output")?;

    match *digit {
        NUMBER_ZERO_IN_ASCII => Ok(false),
        NUMBER_ONE_IN_ASCII => Ok(true),
        _ => panic!("unexpected {digit}"),
    }
}

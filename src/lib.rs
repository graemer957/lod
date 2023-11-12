#[cfg(target_os = "macos")]
mod app_state;
#[cfg(target_os = "macos")]
pub use app_state::{AppState, Mode};
mod command;
#[cfg(target_os = "macos")]
mod menu_item;

use command::Command;
use std::error::Error;

const NUMBER_ZERO_IN_ASCII: u8 = 48;
const NUMBER_ONE_IN_ASCII: u8 = 49;

/// Make use of the `defaults` builtin macOS command line tool to get if the Dock is set to autohide
///
/// **NOTE**: I may look for a better way™ to do this in future - this is a quick hack project after all ;-)
pub fn dock_autohide() -> Result<bool, Box<dyn Error>> {
    let result = Command::new("defaults", &["read", "com.apple.dock", "autohide"]).execute()?;

    if result.stdout().len() != 2 {
        return Err("Got more chars from output than expected".into());
    }
    let digit = result
        .stdout()
        .first()
        .ok_or("Could not get first byte of output")?;

    match *digit {
        NUMBER_ZERO_IN_ASCII => Ok(false),
        NUMBER_ONE_IN_ASCII => Ok(true),
        _ => panic!("Unexpected {digit}"),
    }
}

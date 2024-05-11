#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

#[cfg(target_os = "macos")]
use lod::{AppState, Application, Config, Mode, StateChangeMessage};
use std::{error::Error, sync::mpsc};

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;

    // For me, when I hide my Dock I am in 'laptop' mode
    let mode = if lod::dock_autohide()? {
        Mode::Laptop
    } else {
        Mode::Desktop
    };
    println!("Starting in {mode:#?} mode");

    let (sender, receiver) = mpsc::channel();
    let mut app_state = AppState::new(config, mode, sender);
    Application::run(&receiver, move |message| match message {
        StateChangeMessage::Quit => (),
        StateChangeMessage::ClearCaffeination => {
            app_state.clear_caffeinate();
        }
        StateChangeMessage::ToggleMode => {
            app_state.toggle_mode();
        }
        StateChangeMessage::ToggleCaffeination => {
            app_state.toggle_caffeination();
        }
    });

    Ok(())
}

#[cfg(target_os = "linux")]
fn main() {}

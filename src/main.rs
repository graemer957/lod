#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

#[cfg(target_os = "macos")]
use lod::{AppState, Application, Config, Mode, StateChangeMessage};
use std::{cell::RefCell, error::Error, rc::Rc, sync::mpsc};

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

    // Weakly reference self, in order to use in the `MenuItem` callbacks
    // Bad idea? Most likely, but rolling with it for now™
    let app_state = Rc::new(RefCell::new(AppState::new(config, mode, sender)));
    app_state
        .try_borrow_mut()
        .unwrap()
        .set_weak_self(Rc::downgrade(&app_state));

    Application::run(&receiver, move |message| match message {
        StateChangeMessage::Quit => (),
        StateChangeMessage::ClearCaffeination => {
            app_state.try_borrow_mut().unwrap().clear_caffeinate();
        }
        StateChangeMessage::ToggleMode => {
            app_state.try_borrow_mut().unwrap().toggle_mode();
        }
        StateChangeMessage::ToggleCaffeination => {
            app_state.try_borrow_mut().unwrap().toggle_caffeination();
        }
    });

    Ok(())
}

#[cfg(target_os = "linux")]
fn main() {}

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

#[cfg(target_os = "macos")]
use icrate::AppKit::NSApplication;
#[cfg(target_os = "macos")]
use lod::{AppState, Config, Mode};
use std::{cell::RefCell, error::Error, rc::Rc};

#[cfg(target_os = "macos")]
struct Application;

#[cfg(target_os = "macos")]
impl Application {
    fn run() {
        unsafe {
            NSApplication::sharedApplication().run();
        }
    }
}

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

    // Weakly reference self, in order to use in the `MenuItem` callbacks
    // Bad idea? Most likely, but rolling with it for now™
    let app_state = Rc::new(RefCell::new(AppState::new(config, mode)));
    app_state
        .try_borrow_mut()
        .unwrap()
        .set_weak_self(Rc::downgrade(&app_state));

    Application::run();

    Ok(())
}

#[cfg(target_os = "linux")]
fn main() {}

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

#[cfg(target_os = "macos")]
use lod::{AppState, Mode};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::sync::mpsc::channel;

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn Error>> {
    let (_sender, receiver) = channel::<()>();
    let (event_loop, terminator) = system_status_bar_macos::sync_event_loop(receiver, |()| {});

    // For me, when I hide my Dock I am in 'laptop' mode
    let mode = if lod::dock_autohide()? {
        Mode::Laptop
    } else {
        Mode::Desktop
    };

    // Weakly reference self, in order to use in the `MenuItem` callbacks
    // Bad idea? Most likely, but rolling with it for now™
    let app_state = Rc::new(RefCell::new(AppState::new(terminator, mode)));
    app_state
        .try_borrow_mut()
        .unwrap()
        .set_weak_self(Rc::downgrade(&app_state));

    event_loop();
    Ok(())
}

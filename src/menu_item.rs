use super::AppState;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use system_status_bar_macos::{LoopTerminator, MenuItem};

pub trait Ext {
    fn toggle_mode(title: &str, app_state: Weak<RefCell<AppState>>) -> MenuItem;
    fn quit_item(terminator: Rc<LoopTerminator>) -> MenuItem;
}

impl Ext for MenuItem {
    fn toggle_mode(title: &str, app_state: Weak<RefCell<AppState>>) -> MenuItem {
        Self::new(
            title,
            Some(Box::new(move || {
                println!("toggling mode!");

                app_state
                    .upgrade()
                    .unwrap()
                    .try_borrow_mut()
                    .unwrap()
                    .toggle_mode();
            })),
            None,
        )
    }

    fn quit_item(terminator: Rc<LoopTerminator>) -> MenuItem {
        Self::new(
            "Quit",
            Some(Box::new(move || {
                terminator.terminate();
            })),
            None,
        )
    }
}

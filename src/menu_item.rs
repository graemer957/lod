use super::AppState;
use std::cell::RefCell;
use std::process::exit;
use std::rc::Weak;
use system_status_bar_macos::MenuItem;

pub trait Ext {
    fn toggle_mode(title: &str, app_state: Weak<RefCell<AppState>>) -> MenuItem;
    fn quit_item() -> MenuItem;
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

    fn quit_item() -> MenuItem {
        Self::new(
            "Quit",
            Some(Box::new(move || {
                exit(0);
            })),
            None,
        )
    }
}

use super::AppState;
use std::{cell::RefCell, process::exit, rc::Weak};
use system_status_bar_macos::{MenuItem, MenuItemState};

type AppStateRef = Weak<RefCell<AppState>>;

pub trait Ext {
    fn toggle_mode(
        title: impl AsRef<str>,
        image: impl AsRef<str>,
        accessibility_description: impl AsRef<str>,
        app_state: AppStateRef,
    ) -> MenuItem;

    fn caffinate_item(caffinating: bool, app_state: AppStateRef) -> MenuItem;

    fn quit_item() -> MenuItem;
}

impl Ext for MenuItem {
    fn toggle_mode(
        title: impl AsRef<str>,
        image_named: impl AsRef<str>,
        accessibility_description: impl AsRef<str>,
        app_state: AppStateRef,
    ) -> MenuItem {
        let mut menu_item = Self::new(
            title,
            Some(Box::new(move || {
                app_state
                    .upgrade()
                    .unwrap()
                    .try_borrow_mut()
                    .unwrap()
                    .toggle_mode();
            })),
            None,
        );
        menu_item.set_image_with_system_symbol_name(image_named, Some(accessibility_description));

        menu_item
    }

    fn caffinate_item(caffinating: bool, app_state: AppStateRef) -> MenuItem {
        let mut caffinate_item = Self::new(
            "Caffinate",
            Some(Box::new(move || {
                app_state
                    .upgrade()
                    .unwrap()
                    .try_borrow_mut()
                    .unwrap()
                    .toggle_caffeination();
            })),
            None,
        );
        caffinate_item
            .set_image_with_system_symbol_name("mug.fill", Some("Toggle caffeination of your Mac"));
        if caffinating {
            caffinate_item.set_state(MenuItemState::On);
        } else {
            caffinate_item.set_state(MenuItemState::Off);
        }

        caffinate_item
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

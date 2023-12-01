use super::AppState;
use std::{cell::RefCell, process, rc::Weak};
use system_status_bar_macos::{MenuItem, MenuItemState};

type AppStateRef = Weak<RefCell<AppState>>;

pub trait Ext {
    fn toggle_mode(
        title: impl AsRef<str>,
        image: impl AsRef<str>,
        accessibility_description: impl AsRef<str>,
        app_state: AppStateRef,
    ) -> MenuItem;

    fn caffeinate_item(caffeinating: bool, app_state: AppStateRef) -> MenuItem;

    fn quit_item(app_state: AppStateRef) -> MenuItem;
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

    fn caffeinate_item(caffeinating: bool, app_state: AppStateRef) -> MenuItem {
        let mut caffeinate_item = Self::new(
            "Caffeinate",
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
        caffeinate_item
            .set_image_with_system_symbol_name("mug.fill", Some("Toggle caffeination of your Mac"));
        if caffeinating {
            caffeinate_item.set_state(MenuItemState::On);
        } else {
            caffeinate_item.set_state(MenuItemState::Off);
        }

        caffeinate_item
    }

    fn quit_item(app_state: AppStateRef) -> MenuItem {
        Self::new(
            "Quit",
            Some(Box::new(move || {
                app_state
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .delete_apple_scripts();

                app_state.upgrade().unwrap().borrow_mut().kill_caffeinate();

                process::exit(0);
            })),
            None,
        )
    }
}

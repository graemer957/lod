use super::menu_item::Ext;
use std::{cell::RefCell, rc::Weak};
use system_status_bar_macos::{Menu, MenuItem, StatusItem};

#[derive(Debug)]
pub enum Mode {
    Laptop,
    Desktop,
}

impl Mode {
    const fn accessibility_description(&self) -> &'static str {
        match self {
            Self::Laptop => "Switch to Laptop mode",
            Self::Desktop => "Switch to Desktop mode",
        }
    }

    const fn description(&self) -> &'static str {
        match self {
            Self::Laptop => "Laptop Mode",
            Self::Desktop => "Desktop Mode",
        }
    }

    const fn sf_symbol(&self) -> &'static str {
        match self {
            Self::Laptop => "laptopcomputer",
            Self::Desktop => "desktopcomputer",
        }
    }

    const fn toggle(&self) -> Self {
        match self {
            Self::Laptop => Self::Desktop,
            Self::Desktop => Self::Laptop,
        }
    }
}

pub struct AppState {
    status_item: StatusItem,
    mode: Mode,
    weak_self: Weak<RefCell<Self>>,
    caffinating: bool,
}

impl AppState {
    #[must_use]
    pub fn new(mode: Mode) -> Self {
        let mut status_item = StatusItem::new("", Menu::new(vec![]));
        status_item.set_image_with_system_symbol_name(
            mode.sf_symbol(),
            Some(mode.accessibility_description()),
        );

        Self {
            status_item,
            mode,
            weak_self: Weak::new(),
            caffinating: false,
        }
    }

    pub fn set_weak_self(&mut self, weak_self: Weak<RefCell<Self>>) {
        self.weak_self = weak_self;

        self.configure_menu_items();
    }

    /// # Panics
    ///
    /// If unable to `borrow_mut` on self(!)
    pub fn toggle_mode(&mut self) {
        let new_mode = self.mode.toggle();
        println!("Switching to {new_mode:#?} mode");
        self.status_item.set_image_with_system_symbol_name(
            new_mode.sf_symbol(),
            Some(new_mode.accessibility_description()),
        );
        self.mode = new_mode;

        self.configure_menu_items();
    }

    fn configure_menu_items(&mut self) {
        let opposite_mode = self.mode.toggle();
        let menu_items = vec![
            MenuItem::toggle_mode(
                opposite_mode.description(),
                opposite_mode.sf_symbol(),
                opposite_mode.accessibility_description(),
                self.weak_self.clone(),
            ),
            MenuItem::caffinate_item(self.caffinating, self.weak_self.clone()),
            MenuItem::separator(),
            MenuItem::quit_item(),
        ];
        self.status_item.set_menu(Menu::new(menu_items));
    }

    #[must_use]
    pub const fn caffinating(&self) -> bool {
        self.caffinating
    }

    pub fn toggle_caffeination(&mut self) {
        self.caffinating = !self.caffinating;
        println!("Switching caffeination to {}", self.caffinating);

        self.configure_menu_items();
    }
}

use super::menu_item::Ext;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use system_status_bar_macos::{Menu, MenuItem, StatusItem};

#[derive(Debug)]
pub enum Mode {
    Laptop,
    Desktop,
}

const LAPTOP_CHAR: &str = "💻";
const DESKTOP_CHAR: &str = "🖥";

pub struct AppState {
    status_item: Rc<RefCell<StatusItem>>,
    mode: Mode,
    weak_self: Weak<RefCell<Self>>,
}

impl AppState {
    #[must_use]
    pub fn new(mode: Mode) -> Self {
        let title = match mode {
            Mode::Laptop => LAPTOP_CHAR,
            Mode::Desktop => DESKTOP_CHAR,
        };
        let mut status_item = StatusItem::new("A title", Menu::new(vec![]));
        status_item.set_image_with_system_symbol_name("laptopcomputer", Some(""));

        Self {
            status_item: Rc::new(RefCell::new(status_item)),
            mode,
            weak_self: Weak::new(),
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
        let mode = match self.mode {
            Mode::Laptop => {
                self.mode = Mode::Desktop;
                DESKTOP_CHAR
            }
            Mode::Desktop => {
                self.mode = Mode::Laptop;
                LAPTOP_CHAR
            }
        };
        println!("Switching to {:#?} mode", self.mode);
        self.status_item.try_borrow_mut().unwrap().set_title(mode);

        self.configure_menu_items();
    }

    fn configure_menu_items(&self) {
        let cloned_self = self.weak_self.clone();
        let menu_items = vec![
            match self.mode {
                Mode::Laptop => MenuItem::toggle_mode(
                    "Desktop Mode",
                    "desktopcomputer",
                    "Switch to Desktop mode",
                    cloned_self,
                ),
                Mode::Desktop => MenuItem::toggle_mode(
                    "Laptop Mode",
                    "laptopcomputer",
                    "Switch to Laptop mode",
                    cloned_self,
                ),
            },
            MenuItem::separator(),
            MenuItem::quit_item(),
        ];
        self.status_item
            .try_borrow_mut()
            .unwrap()
            .set_menu(Menu::new(menu_items));
    }
}

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
    caffinating: bool,
}

impl AppState {
    #[must_use]
    pub fn new(mode: Mode) -> Self {
        let mut status_item = StatusItem::new("", Menu::new(vec![]));
        status_item.set_image_with_system_symbol_name("laptopcomputer", Some(""));

        Self {
            status_item: Rc::new(RefCell::new(status_item)),
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
        let menu_items = vec![
            MenuItem::toggle_mode(
                match self.mode {
                    Mode::Laptop => "Desktop Mode",
                    Mode::Desktop => "Laptop Mode",
                },
                self.mode.sf_symbol(),
                self.mode.accessibility_description(),
                self.weak_self.clone(),
            ),
            MenuItem::caffinate_item(self.caffinating, self.weak_self.clone()),
            MenuItem::separator(),
            MenuItem::quit_item(),
        ];
        self.status_item
            .try_borrow_mut()
            .unwrap()
            .set_menu(Menu::new(menu_items));
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

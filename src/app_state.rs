use super::menu_item::MenuItemExt;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use system_status_bar_macos::*;

pub enum Mode {
    Laptop,
    Desktop,
}

const LAPTOP_CHAR: &str = "💻";
const DESKTOP_CHAR: &str = "🖥";

pub struct AppState {
    terminator: Rc<LoopTerminator>,
    status_item: Rc<RefCell<StatusItem>>,
    mode: Mode,
    weak_self: Weak<RefCell<Self>>,
}

impl AppState {
    pub fn new(terminator: LoopTerminator, mode: Mode) -> Self {
        let terminator = Rc::new(terminator);

        let title = match mode {
            Mode::Laptop => LAPTOP_CHAR,
            Mode::Desktop => DESKTOP_CHAR,
        };
        let status_item = Rc::new(RefCell::new(StatusItem::new(title, Menu::new(vec![]))));

        AppState {
            terminator,
            status_item,
            mode,
            weak_self: Weak::new(),
        }
    }

    pub fn set_weak_self(&mut self, weak_self: Weak<RefCell<Self>>) {
        self.weak_self = weak_self;

        self.configure_menu_items();
    }

    pub fn toggle_mode(&mut self) {
        match self.mode {
            Mode::Laptop => {
                self.mode = Mode::Desktop;

                self.status_item.try_borrow_mut().unwrap().set_title("🖥️");
            }
            Mode::Desktop => {
                self.mode = Mode::Laptop;

                self.status_item.try_borrow_mut().unwrap().set_title("💻");
            }
        };

        self.configure_menu_items();
    }

    fn configure_menu_items(&self) {
        let cloned_self = self.weak_self.clone();
        let menu_items = match self.mode {
            Mode::Laptop => vec![
                MenuItem::toggle_mode(DESKTOP_CHAR, cloned_self),
                MenuItem::quit_item(self.terminator.clone()),
            ],
            Mode::Desktop => vec![
                MenuItem::toggle_mode(LAPTOP_CHAR, cloned_self),
                MenuItem::quit_item(self.terminator.clone()),
            ],
        };
        self.status_item
            .try_borrow_mut()
            .unwrap()
            .set_menu(Menu::new(menu_items));
    }
}

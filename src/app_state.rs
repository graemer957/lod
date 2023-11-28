use super::{
    menu_item::Ext,
    program::{Program, ProgramImpl},
    Config,
};
use std::{cell::RefCell, process::Command, rc::Weak};
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
    config: Config,
    status_item: StatusItem,
    mode: Mode,
    weak_self: Weak<RefCell<Self>>,
    caffeinating: bool,
}

impl AppState {
    #[must_use]
    pub fn new(config: Config, mode: Mode) -> Self {
        let mut status_item = StatusItem::new("", Menu::new(vec![]));
        status_item.set_image_with_system_symbol_name(
            mode.sf_symbol(),
            Some(mode.accessibility_description()),
        );

        Self {
            config,
            status_item,
            mode,
            weak_self: Weak::new(),
            caffeinating: false,
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

        self.run_apple_script();
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
            MenuItem::caffeinate_item(self.caffeinating, self.weak_self.clone()),
            MenuItem::separator(),
            MenuItem::quit_item(self.weak_self.clone()),
        ];
        self.status_item.set_menu(Menu::new(menu_items));
    }

    #[must_use]
    pub const fn caffeinating(&self) -> bool {
        self.caffeinating
    }

    pub fn toggle_caffeination(&mut self) {
        self.caffeinating = !self.caffeinating;
        println!("Switching caffeination to {}", self.caffeinating);

        self.configure_menu_items();
    }

    pub fn delete_apple_scripts(&mut self) {
        self.config.delete_apple_scripts();
    }

    fn run_apple_script(&self) {
        let mut defaults = Command::new("osascript");
        defaults.args(match self.mode {
            Mode::Laptop => self.config.laptop_applescript_path(),
            Mode::Desktop => self.config.desktop_applescript_path(),
        });

        if let Err(error) = ProgramImpl::new(defaults, 0).execute() {
            eprintln!("{error:?}");
        }
    }
}

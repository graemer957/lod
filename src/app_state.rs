use super::{
    Config,
    menu_item::Ext,
    program::{Program, ProgramImpl},
    waiting_child::WaitingChild,
};
use std::{process::Command, sync::mpsc::Sender, thread};
use system_status_bar_macos::{Image, Menu, MenuItem, StatusItem};

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
    caffeinate: Option<WaitingChild>,
    sender: Sender<StateChangeMessage>,
}

impl AppState {
    #[must_use]
    pub fn new(config: Config, mode: Mode, sender: Sender<StateChangeMessage>) -> Self {
        let mut status_item = StatusItem::new("", Menu::new(vec![]));
        if let Some(image) =
            Image::with_system_symbol_name(mode.sf_symbol(), Some(mode.accessibility_description()))
        {
            status_item.set_image(image);
        }

        let mut app_state = Self {
            config,
            status_item,
            mode,
            caffeinate: None,
            sender,
        };
        app_state.configure_menu_items();
        app_state
    }

    pub fn toggle_mode(&mut self) {
        let new_mode = self.mode.toggle();
        println!("Switching to {new_mode:#?} mode");
        if let Some(image) = Image::with_system_symbol_name(
            new_mode.sf_symbol(),
            Some(new_mode.accessibility_description()),
        ) {
            self.status_item.set_image(image);
        }
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
                self.sender.clone(),
            ),
            MenuItem::caffeinate_item(self.caffeinate.is_some(), self.sender.clone()),
            MenuItem::separator(),
            MenuItem::quit_item(self.sender.clone()),
        ];
        self.status_item.set_menu(Menu::new(menu_items));
    }

    #[must_use]
    pub const fn caffeinating(&self) -> bool {
        self.caffeinate.is_some()
    }

    #[allow(clippy::nonminimal_bool)]
    pub fn toggle_caffeination(&mut self) {
        // We want to show the state we are going to, thus the negation
        // and need for the clippy allow
        println!("Switching caffeination to {}", !self.caffeinate.is_some());

        if self.caffeinate.is_some() {
            self.kill_caffeinate();
        } else {
            let caffeinate_app = self.config.caffeinate_app().unwrap_or("caffeinate");
            let mut caffeinate = Command::new(caffeinate_app);
            if let Some(arg) = self.config.caffeinate_options() {
                caffeinate.arg(arg);
            }
            match caffeinate.spawn() {
                Ok(child) => {
                    let waiting_child = WaitingChild::new(child, self.sender.clone());
                    self.caffeinate = Some(waiting_child);
                }
                Err(error) => {
                    dbg!(error);
                }
            }
        }

        self.configure_menu_items();
    }

    fn run_apple_script(&self) {
        let mut defaults = Command::new("osascript");
        defaults.arg(match self.mode {
            Mode::Laptop => self.config.laptop_applescript_path(),
            Mode::Desktop => self.config.desktop_applescript_path(),
        });

        thread::spawn(move || {
            if let Err(error) = ProgramImpl::new(defaults, 0).execute() {
                eprintln!("{error:?}");
            }
        });
    }

    fn kill_caffeinate(&mut self) {
        if let Some(child) = self.caffeinate.take() {
            if let Err(error) = child.kill() {
                dbg!(error);
            }
        }
    }

    pub fn clear_caffeinate(&mut self) {
        println!(
            "Caffeinate has been killed, updating menu state. NOTE: This message could be delayed \
            from when the process was actually killed as waited until next event loop invocation."
        );
        self.caffeinate.take();
        self.configure_menu_items();
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        println!("Deleting AppleScripts in AppState::drop()");
        self.config.delete_apple_scripts();

        println!("Killing caffeinate");
        self.kill_caffeinate();
    }
}

/// Message sent to change the app's state
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StateChangeMessage {
    /// Toggle the current mode
    ToggleMode,

    /// Toggle caffeination
    ToggleCaffeination,

    /// Clear the caffeination checkmark
    ClearCaffeination,

    /// Quit the app
    Quit,
}

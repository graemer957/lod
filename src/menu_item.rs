use super::app_state::StateChangeMessage;
use std::sync::mpsc::Sender;
use system_status_bar_macos::{ControlState, Image, MenuItem};

pub trait Ext {
    fn toggle_mode(
        title: impl AsRef<str>,
        image: impl AsRef<str>,
        accessibility_description: impl AsRef<str>,
        sender: Sender<StateChangeMessage>,
    ) -> MenuItem;

    fn caffeinate_item(caffeinating: bool, sender: Sender<StateChangeMessage>) -> MenuItem;

    fn quit_item(sender: Sender<StateChangeMessage>) -> MenuItem;
}

impl Ext for MenuItem {
    fn toggle_mode(
        title: impl AsRef<str>,
        image_named: impl AsRef<str>,
        accessibility_description: impl AsRef<str>,
        sender: Sender<StateChangeMessage>,
    ) -> MenuItem {
        let mut menu_item = Self::new(
            title,
            Some(Box::new(move || {
                if let Err(error) = sender.send(StateChangeMessage::ToggleMode) {
                    eprintln!(
                        "Failed to send StateChangeMessage::ToggleMode message. Error: {error}"
                    );
                }
            })),
            None,
        );
        if let Some(image) =
            Image::with_system_symbol_name(image_named, Some(accessibility_description))
        {
            menu_item.set_image(image);
        }

        menu_item
    }

    fn caffeinate_item(caffeinating: bool, sender: Sender<StateChangeMessage>) -> MenuItem {
        let mut caffeinate_item = Self::new(
            "Caffeinate",
            Some(Box::new(move || {
                if let Err(error) = sender.send(StateChangeMessage::ToggleCaffeination) {
                    eprintln!(
                        "Failed to send StateChangeMessage::ToggleCaffeination message. Error: {error}"
                    );
                }
            })),
            None,
        );

        if let Some(image) =
            Image::with_system_symbol_name("mug.fill", Some("Toggle caffeination of your Mac"))
        {
            caffeinate_item.set_image(image);
        }
        if caffeinating {
            caffeinate_item.set_control_state(ControlState::On);
        } else {
            caffeinate_item.set_control_state(ControlState::Off);
        }

        caffeinate_item
    }

    fn quit_item(sender: Sender<StateChangeMessage>) -> MenuItem {
        Self::new(
            "Quit",
            Some(Box::new(move || {
                if let Err(error) = sender.send(StateChangeMessage::Quit) {
                    eprintln!("Failed to send StateChangeMessage::Quit message. Error: {error}");
                }
            })),
            None,
        )
    }
}

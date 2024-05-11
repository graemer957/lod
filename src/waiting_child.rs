use super::app_state::StateChangeMessage;
use crate::program::{Program, ProgramImpl};
use std::{
    error::Error,
    process::{Child, Command},
    sync::mpsc::Sender,
    thread,
};

pub struct WaitingChild {
    id: u32,
}

impl WaitingChild {
    pub fn new(mut child: Child, sender: Sender<StateChangeMessage>) -> Self {
        let result = Self { id: child.id() };

        thread::spawn(move || {
            // We need to `wait` on the child process, otherwise it hangs around on macOS as a
            // zombie. See https://doc.rust-lang.org/std/process/struct.Child.html#warning
            let _ = child.wait();

            if let Err(error) = sender.send(StateChangeMessage::ClearCaffeination) {
                eprintln!(
                    "Failed to send StateChangeMessage::ClearCaffeination message. Error: {error}"
                );
            }
        });

        result
    }

    pub fn kill(&self) -> Result<(), Box<dyn Error>> {
        let mut command = Command::new("kill");
        command.args(["-9", &self.id.to_string()]);
        let mut program = ProgramImpl::new(command, 0);
        program.execute().map(|_| Ok(()))?
    }
}

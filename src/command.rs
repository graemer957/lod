use std::fmt::Formatter;
use std::{
    fmt::{Debug, Display},
    io,
};

pub struct Command<'a> {
    program: &'a str,
    args: &'a [&'a str],
}

impl<'a> Command<'a> {
    pub fn new(program: &'a str, args: &'a [&'a str]) -> Self {
        Command { program, args }
    }

    pub fn execute(self) -> Result<Output, Error> {
        let output = std::process::Command::new(self.program)
            .args(self.args)
            .output()?;

        let status_code = output.status.code().ok_or(Error::BadStatusCode)?;
        let result = Output {
            status_code,
            stdout: output.stdout,
            stderr: output.stderr,
        };

        if status_code != 0 {
            return Err(result.into());
        }
        Ok(result)
    }
}

pub struct Output {
    status_code: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl Output {
    pub fn stdout(&self) -> &Vec<u8> {
        &self.stdout
    }
}

pub enum Error {
    /// `Command` exited with a non-zero status code
    NonZeroStatusCode(Output),

    /// `Command` encountered an I/O error
    Io(io::Error),

    /// Unable to get the status code from the `Command`
    BadStatusCode,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NonZeroStatusCode(result) => {
                write!(f, "Non-zero status code: {}", result.status_code)
            }
            Error::Io(error) => write!(f, "{error}"),
            Error::BadStatusCode => write!(f, "Unable to get status code from `Command`"),
        }
    }
}

// Implemented manually, because I found `ExitStatus` does not show the underlying
// status code properly.
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NonZeroStatusCode(result) => {
                let status_code = result.status_code;
                let stdout = String::from_utf8(result.stdout.clone())
                    .unwrap_or_else(|error| format!("{error}"));
                let stderr = String::from_utf8(result.stderr.clone())
                    .unwrap_or_else(|error| format!("{error}"));

                write!(
                    f,
                    "Unexpected status code: {status_code} | stdout: {stdout:?} | stderr: {stderr:?}"
                )
            }
            Error::Io(error) => write!(f, "{error:?}"),
            Error::BadStatusCode => write!(f, "Unable to get status code from `Command`"),
        }
    }
}

impl std::error::Error for Error {}

impl From<Output> for Error {
    fn from(result: Output) -> Self {
        Error::NonZeroStatusCode(result)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

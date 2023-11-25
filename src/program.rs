#[cfg(test)]
use mockall::automock;
use std::{
    fmt::{Debug, Display, Formatter},
    io,
};

#[allow(clippy::module_name_repetitions)]
pub struct ProgramImpl<T: Command> {
    command: T,
    expected_status_code: i32,
}

impl<T> ProgramImpl<T>
where
    T: Command,
{
    pub const fn new(command: T, expected_status_code: i32) -> Self {
        Self {
            command,
            expected_status_code,
        }
    }
}

pub trait Program {
    /// Executes specified `Command`
    ///
    /// # Errors
    ///
    /// Returns an error describing why the `Command` failed
    fn execute(&mut self) -> Result<Output, Error>;
}

impl<T> Program for ProgramImpl<T>
where
    T: Command,
{
    fn execute(&mut self) -> Result<Output, Error> {
        let output = self.command.output()?;
        let status_code = output.status.code().ok_or(Error::NoStatusCode)?;
        let result = Output {
            status_code,
            stdout: output.stdout,
            stderr: output.stderr,
        };

        if status_code != self.expected_status_code {
            return Err(result.into());
        }
        Ok(result)
    }
}

pub enum Error {
    /// `Command` encountered an I/O error
    Io(io::Error),

    /// Unable to get the status code from the `Command`
    NoStatusCode,

    /// `Command` exited with an unexpected status code
    UnexpectedStatusCode(Output),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "{error}"),
            Self::NoStatusCode => write!(f, "Unable to get status code from `Command`"),
            Self::UnexpectedStatusCode(result) => {
                write!(f, "Unexpected status code: {}", result.status_code)
            }
        }
    }
}

// Implemented manually, because I found `ExitStatus` does not show the underlying
// status code properly.
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "{error:?}"),
            Self::NoStatusCode => write!(f, "Unable to get status code from `Command`"),
            Self::UnexpectedStatusCode(result) => {
                let status_code = result.status_code;
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);
                write!(
                    f,
                    "Unexpected status code: {status_code} | stdout: {stdout:?} | stderr: {stderr:?}"
                )
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<Output> for Error {
    fn from(result: Output) -> Self {
        Self::UnexpectedStatusCode(result)
    }
}

#[cfg_attr(test, automock)]
pub trait Command {
    /// Gets the output from specified `Command`
    ///
    /// # Errors
    ///
    /// Returns an error describing why the output could not be retrieved
    fn output(&mut self) -> io::Result<std::process::Output>;
}

impl Command for std::process::Command {
    fn output(&mut self) -> io::Result<std::process::Output> {
        self.output()
    }
}

#[derive(Debug)]
pub struct Output {
    status_code: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl Output {
    #[must_use]
    pub const fn status_code(&self) -> &i32 {
        &self.status_code
    }

    #[must_use]
    pub fn stdout(&self) -> &[u8] {
        &self.stdout[..]
    }

    #[must_use]
    pub fn stderr(&self) -> &[u8] {
        &self.stderr[..]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::ErrorKind;
    use std::os::unix::prelude::ExitStatusExt;

    #[test]
    fn it_works() {
        let mut mock = MockCommand::new();
        mock.expect_output().times(1).returning(|| {
            Ok(std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: vec![],
                stderr: vec![],
            })
        });

        let sut = ProgramImpl::new(mock, 0).execute().unwrap();
        assert_eq!(sut.status_code, 0);
    }

    #[test]
    fn it_has_stdout() {
        let mut mock = MockCommand::new();
        mock.expect_output().times(1).returning(|| {
            Ok(std::process::Output {
                status: std::process::ExitStatus::default(),
                stdout: vec![9, 4, 22],
                stderr: vec![],
            })
        });

        let sut = ProgramImpl::new(mock, 0).execute().unwrap();
        assert_eq!(sut.stdout(), &[9, 4, 22]);
    }

    #[test]
    fn it_has_stderr() {
        let mut mock = MockCommand::new();
        mock.expect_output().times(1).returning(|| {
            Ok(std::process::Output {
                status: std::process::ExitStatus::default(),
                stdout: vec![],
                stderr: vec![9, 4, 22],
            })
        });

        let sut = ProgramImpl::new(mock, 0).execute().unwrap();
        assert_eq!(sut.stderr(), &[9, 4, 22]);
    }

    #[test]
    fn it_returns_no_status_code() {
        let mut mock = MockCommand::new();
        mock.expect_output().times(1).returning(|| {
            Ok(std::process::Output {
                // By chance I found out that any int other than 0 will cause
                // `output.status.code()` to return an error, thus we capture
                // as `Error::NoStatusCode`
                status: std::process::ExitStatus::from_raw(1),
                stdout: vec![],
                stderr: vec![],
            })
        });

        let sut = ProgramImpl::new(mock, 0).execute();
        assert!(matches!(sut, Err(Error::NoStatusCode)));
    }

    #[test]
    fn it_returns_no_status_code_with_correct_display_and_debug() {
        let mut mock = MockCommand::new();
        mock.expect_output().times(1).returning(|| {
            Ok(std::process::Output {
                // By chance I found out that any int other than 0 will cause
                // `output.status.code()` to return an error, thus we capture
                // as `Error::NoStatusCode`
                status: std::process::ExitStatus::from_raw(1),
                stdout: vec![],
                stderr: vec![],
            })
        });

        let sut = ProgramImpl::new(mock, 0).execute().err().unwrap();
        assert_eq!(format!("{sut}"), "Unable to get status code from `Command`");
        assert_eq!(
            format!("{sut:?}"),
            "Unable to get status code from `Command`"
        );
    }

    #[test]
    fn it_returns_io_error() {
        let mut mock = MockCommand::new();
        mock.expect_output()
            .times(1)
            .returning(|| Err(io::Error::other("intentional error")));

        let sut = ProgramImpl::new(mock, 0).execute();
        assert!(matches!(sut, Err(Error::Io(error)) if error.kind() == io::ErrorKind::Other));
    }

    #[test]
    fn it_returns_io_error_with_correct_display_and_debug() {
        let mut mock = MockCommand::new();
        mock.expect_output()
            .times(1)
            .returning(|| Err(io::Error::from(ErrorKind::NotFound)));

        let sut = ProgramImpl::new(mock, 0).execute().err().unwrap();
        assert_eq!(format!("{sut}"), "entity not found");
        assert_eq!(format!("{sut:?}"), "Kind(NotFound)");
    }

    #[test]
    fn it_returns_unexpected_status_code() {
        let mut mock = MockCommand::new();
        mock.expect_output().times(1).returning(|| {
            Ok(std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: vec![],
                stderr: vec![],
            })
        });

        let sut = ProgramImpl::new(mock, 1).execute();
        assert!(matches!(sut, Err(Error::UnexpectedStatusCode(_))));
    }

    #[test]
    fn it_returns_unexpected_status_code_with_correct_display_and_debug() {
        let mut mock = MockCommand::new();
        mock.expect_output().times(1).returning(|| {
            Ok(std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: String::from("stdout: nothing useful!").into_bytes(),
                stderr: String::from("does stderr help?").into_bytes(),
            })
        });

        let sut = ProgramImpl::new(mock, 1).execute().err().unwrap();
        assert_eq!(format!("{sut}"), "Unexpected status code: 0");
        assert_eq!(
            format!("{sut:?}"),
            "Unexpected status code: 0 | stdout: \"stdout: nothing useful!\" | stderr: \"does stderr help?\""
        );
    }
}

use lod::program::{Error, Program, ProgramImpl};
use std::{io::ErrorKind, process::Command};

#[test]
fn it_works() -> Result<(), Error> {
    let r#true = Command::new("true");
    let output = ProgramImpl::new(r#true, 0).execute()?;
    assert_eq!(output.status_code(), &0);
    assert_eq!(output.stdout(), []);
    assert_eq!(output.stderr(), []);

    Ok(())
}

#[test]
fn it_fails() {
    let r#true = Command::new("___zzz");
    let output = ProgramImpl::new(r#true, 0).execute();
    assert!(matches!(output.err(), Some(Error::Io(error)) if error.kind() == ErrorKind::NotFound));
}

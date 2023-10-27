use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    loop {
        let mut command = String::new();
        stdin.read_line(&mut command)?;

        match command[..].trim_end() {
            "quit" => break,
            "dock" => {
                if lod::dock_autohide()? {
                    println!("dock is set to autohide");
                } else {
                    println!("dock is NOT set to autohide");
                }
            }
            _ => println!("Unknown command: {command}"),
        }
    }

    Ok(())
}

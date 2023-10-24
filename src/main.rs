use std::io;

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();

    loop {
        let mut command = String::new();
        stdin.read_line(&mut command)?;

        match command[..].trim_end() {
            "quit" => break,
            "dock" => println!("you want to do something with the dock?"),
            _ => println!("Unknown command: {command}"),
        }
    }

    Ok(())
}

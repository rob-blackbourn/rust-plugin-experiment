use std::io::{self, Write};

fn main() -> io::Result<()> {

    let args: Vec<String> = std::env::args().collect();
    eprintln!("plugin: starting. args={args:#?}");

    let mut ok = true;
    while ok {
        let mut buffer = String::new();
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            ok = false;
            continue;
        }
        eprintln!("plugin: received and echoing \"{}\"", buffer.trim());
        io::stdout().write_all(buffer.as_bytes())?;
    }

    eprintln!("plugin: exiting");

    Ok(())
}

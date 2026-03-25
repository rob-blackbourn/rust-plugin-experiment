use std::io::{self, Write};

fn main() -> io::Result<()> {
    eprintln!("plugin: starting");

    loop {
        let Some(line) = read_stdin()? else {
            break;
        };

        eprintln!("plugin: received and echoing \"{}\"", line.trim());
        write_stdout(&line)?
    }

    eprintln!("plugin: exiting");

    Ok(())
}

fn read_stdin() -> io::Result<Option<String>> {
    let mut line = String::new();
    let bytes_read = io::stdin().read_line(&mut line)?;
    if bytes_read == 0 {
        Ok(None)
    } else {
        Ok(Some(line))
    }
}

fn write_stdout(line: &str) -> io::Result<()> {
    io::stdout().write_all(line.as_bytes())
}

use std::io::{self, Write};

pub fn read_stdin() -> io::Result<Option<String>> {
    let mut line = String::new();
    let bytes_read = io::stdin().read_line(&mut line)?;
    if bytes_read == 0 {
        Ok(None)
    } else {
        Ok(Some(line))
    }
}

pub fn write_stdout(line: &str) -> io::Result<()> {
    io::stdout().write_all(line.as_bytes())
}

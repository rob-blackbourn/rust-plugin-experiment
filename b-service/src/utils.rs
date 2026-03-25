use std::io::BufReader;
use std::io::{self, BufRead, Write};
use std::process::{ChildStdin, ChildStdout};

pub fn read_stdin() -> io::Result<Option<String>> {
    let mut line = String::new();
    let bytes_read = io::stdin().read_line(&mut line)?;
    if bytes_read == 0 {
        Ok(None)
    } else {
        Ok(Some(line))
    }
}

pub fn send_to_plugin(stdin: &mut ChildStdin, line: &str) -> io::Result<()> {
    stdin.write(line.as_bytes())?;
    Ok(())
}

pub fn receive_from_plugin(reader: &mut BufReader<ChildStdout>) -> io::Result<String> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(line)
}

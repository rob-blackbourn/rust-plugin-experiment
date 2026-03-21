use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let mut plugin = Command::new("./example-plugin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("should start plugin");

    let mut plugin_stdin = plugin.stdin.take().expect("should open plugin stdin");
    let mut plugin_stdout = plugin.stdout.take().expect("should open plugin stdout");


    let mut buffer = String::new();
    let mut ok = true;
    while ok {
        println!("Type \"EXIT\" to quit");
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            ok = false;
            continue;
        }
        let line = buffer.trim();
        if bytes_read == 0 || line == "EXIT" {
            ok = false;
            continue;
        }
        println!("line: \"{}\"", line);
        plugin_stdin.write(buffer.as_bytes())?;
        plugin_stdout.read_to_string(&mut buffer)?;
        println!("Received: {}", buffer);
    }

    println!("Existed normally");

    Ok(())
}

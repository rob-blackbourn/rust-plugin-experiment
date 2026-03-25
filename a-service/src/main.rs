use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

fn main() -> io::Result<()> {
    let mut plugin = Plugin::new();

    loop {
        println!("service: enter CTRL-D to quit");
        let Some(line) = read_stdin()? else {
            break;
        };

        println!("service: sending \"{}\"", line.trim());
        plugin.stdin.write(line.as_bytes())?;

        let mut plugin_buffer = String::new();
        plugin.reader.read_line(&mut plugin_buffer)?;
        println!("service: received: {}", plugin_buffer.trim());
    }

    println!("service: existed normally");

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

struct Plugin {
    stdin: ChildStdin,
    reader: BufReader<ChildStdout>,
}

impl Plugin {
    pub fn new() -> Self {
        let mut child = Command::new("./a-plugin")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("service: should start plugin");

        let stdin = child
            .stdin
            .take()
            .expect("service: should open plugin stdin");
        let stdout = child
            .stdout
            .take()
            .expect("service: should open plugin stdout");
        let reader = BufReader::new(stdout);

        Self { stdin, reader }
    }
}

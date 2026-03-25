use std::io::BufReader;
use std::process::{ChildStdin, ChildStdout, Command, Stdio};

pub struct Plugin {
    pub stdin: ChildStdin,
    pub reader: BufReader<ChildStdout>,
}

impl Plugin {
    pub fn new() -> Self {
        let mut child = Command::new("b-plugin")
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

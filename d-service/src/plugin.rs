use std::io::{self, BufReader};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};

pub struct Plugin {
    pub stdin: ChildStdin,
    pub reader: BufReader<ChildStdout>,
}

impl Plugin {
    pub fn new(cmdline: Vec<String>) -> io::Result<Self> {
        let mut child = Command::new("d-plugin")
            .args(cmdline)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to open stdin"))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to open stdout"))?;

        let reader = BufReader::new(stdout);

        Ok(Self { stdin, reader })
    }
}

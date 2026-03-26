use std::io::{self, BufReader};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};

pub struct Plugin {
    pub stdin: ChildStdin,
    pub reader: BufReader<ChildStdout>,
}

impl Plugin {
    pub fn new(cmdline: Vec<String>) -> io::Result<Self> {
        let child = Command::new("d-plugin")
            .args(cmdline)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let Some(stdin) = child.stdin else {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to open stdin"));
        };

        let Some(stdout) = child.stdout else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to open stdout",
            ));
        };

        Ok(Self {
            stdin,
            reader: BufReader::new(stdout),
        })
    }
}

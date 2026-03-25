use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::{ChildStdin, ChildStdout, Command, Stdio};

fn main() -> io::Result<()> {
    add_current_exe_dir_to_path();

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

fn add_plugin_path(path: &Path) -> () {
    let key = "PATH";
    match env::var_os(key) {
        Some(path_var) => {
            let mut paths = env::split_paths(&path_var).collect::<Vec<_>>();
            paths.push(path.into());
            let new_path = env::join_paths(paths).expect("should join paths");
            unsafe {
                env::set_var(key, &new_path);
            }
        }
        None => {
            let paths: Vec<&Path> = vec![path];
            let path_var = env::join_paths(paths).expect("should join path");
            unsafe {
                env::set_var(key, &path_var);
            }
        }
    }
}

fn add_current_exe_dir_to_path() -> () {
    let mut path = env::current_exe().expect("should find exe dir");
    path.pop();
    add_plugin_path(path.as_path());
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

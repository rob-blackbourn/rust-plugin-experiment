use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};

mod args;
use args::Args;

fn add_plugin_path(path: &str) -> () {
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
            let paths = vec![Path::new(path)];
            let path_var = env::join_paths(paths.iter()).expect("should join path");
            unsafe {
                env::set_var(key, &path_var);
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args = Args::load()?;

    add_plugin_path(&args.plugin_path);

    let mut plugin = Command::new("d-plugin")
        .args(args.plugin_cmdline)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("service: should start plugin");

    let mut plugin_stdin = plugin
        .stdin
        .take()
        .expect("service: should open plugin stdin");
    let plugin_stdout = plugin
        .stdout
        .take()
        .expect("service: should open plugin stdout");
    let mut plugin_reader = BufReader::new(plugin_stdout);

    let mut ok = true;
    while ok {
        println!("service: enter CTRL-D to quit");

        let mut input_buffer = String::new();
        let bytes_read = io::stdin().read_line(&mut input_buffer)?;
        if bytes_read == 0 {
            ok = false;
            continue;
        }

        println!("service: sending \"{}\"", input_buffer.trim());
        plugin_stdin.write(input_buffer.as_bytes())?;

        let mut plugin_buffer = String::new();
        plugin_reader.read_line(&mut plugin_buffer)?;
        println!("service: received: {}", plugin_buffer.trim());
    }

    println!("service: existed normally");

    Ok(())
}

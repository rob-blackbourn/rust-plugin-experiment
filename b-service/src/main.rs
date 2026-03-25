use std::env;
use std::io;
use std::path::Path;

mod plugin;
use plugin::Plugin;

mod utils;
use utils::{read_stdin, receive_from_plugin, send_to_plugin};

fn main() -> io::Result<()> {
    add_current_exe_dir_to_path();

    let mut plugin = Plugin::new();

    loop {
        println!("service: enter CTRL-D to quit");

        let Some(line) = read_stdin()? else {
            break;
        };

        println!("service: sending \"{}\"", line.trim());
        send_to_plugin(&mut plugin.stdin, &line)?;

        let response = receive_from_plugin(&mut plugin.reader)?;
        println!("service: received: {}", response.trim());
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

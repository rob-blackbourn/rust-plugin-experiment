use std::env;
use std::io::{self, BufRead, Write};
use std::path::Path;

use d_common::{Credentials, Status};

mod args;
use args::Args;

mod plugin;
use plugin::Plugin;

fn main() -> io::Result<()> {
    let args = Args::load()?;

    add_plugin_path(&args.plugin_path);

    let mut plugin = Plugin::new(args.plugin_cmdline);

    let credentials = read_credentials()?;

    send_credentials_to_plugin(&credentials, &mut plugin)?;

    let status = receive_status_from_plugin(&mut plugin)?;

    match status.ok {
        true => println!("service: authenticated"),
        false => println!("service: incorrect username or password"),
    }

    Ok(())
}

fn read_credentials() -> io::Result<Credentials> {
    let username = input("username: ")?;
    let password = input("password: ")?;

    Ok(Credentials {
        username: username.trim().to_string(),
        password: password.trim().to_string(),
    })
}

fn send_credentials_to_plugin(credentials: &Credentials, plugin: &mut Plugin) -> io::Result<()> {
    let text = serde_json::to_string(&credentials)?;

    plugin.stdin.write(text.as_bytes())?;
    plugin.stdin.write(b"\n")?;

    Ok(())
}

fn receive_status_from_plugin(plugin: &mut Plugin) -> io::Result<Status> {
    let mut plugin_buffer = String::new();
    plugin.reader.read_line(&mut plugin_buffer)?;
    let status: Status = serde_json::from_str(&plugin_buffer)?;
    Ok(status)
}

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

fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut text = String::new();
    let bytes_read = io::stdin().read_line(&mut text)?;
    if bytes_read == 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "Exit"));
    }
    Ok(text)
}

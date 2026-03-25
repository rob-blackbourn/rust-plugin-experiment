use std::io::{self, BufRead, Write};

use d_common::{Credentials, Status};

mod args;
use args::Args;

mod plugin;
use plugin::Plugin;

mod utils;
use utils::{add_plugin_path, input};

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

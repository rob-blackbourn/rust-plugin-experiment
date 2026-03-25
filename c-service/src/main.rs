use std::io;

mod args;
use args::Args;

mod plugin;
use plugin::Plugin;

mod utils;
use utils::{add_plugin_path, read_stdin, receive_from_plugin, send_to_plugin};

fn main() -> io::Result<()> {
    let args = Args::load()?;

    add_plugin_path(&args.plugin_path);

    let mut plugin = Plugin::new(args.plugin_cmdline);

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

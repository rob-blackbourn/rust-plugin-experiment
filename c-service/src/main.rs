use std::io::{self, BufRead, Write};

mod args;
use args::Args;

mod plugin;
use plugin::Plugin;

mod utils;
use utils::{add_plugin_path, read_stdin};

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
        plugin.stdin.write(line.as_bytes())?;

        let mut plugin_buffer = String::new();
        plugin.reader.read_line(&mut plugin_buffer)?;
        println!("service: received: {}", plugin_buffer.trim());
    }

    println!("service: existed normally");

    Ok(())
}

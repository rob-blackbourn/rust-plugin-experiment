use std::io;

mod utils;
use utils::{read_stdin, write_stdout};

fn main() -> io::Result<()> {
    eprintln!("plugin: starting");

    loop {
        let Some(line) = read_stdin()? else {
            break;
        };

        eprintln!("plugin: received and echoing \"{}\"", line.trim());
        write_stdout(&line)?
    }

    eprintln!("plugin: exiting");

    Ok(())
}

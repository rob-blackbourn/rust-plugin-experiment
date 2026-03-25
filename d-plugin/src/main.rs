use std::io::{self, Write};

use d_common::{Credentials, Status};

mod args;
use args::Args;

mod authenticator;
use authenticator::HtpasswdAuthenticator;

fn main() -> io::Result<()> {
    let args = Args::load()?;
    let authenticator = HtpasswdAuthenticator::new(&args.password_file)?;

    eprintln!("plugin: starting. args={args:#?}");

    let mut ok = true;
    while ok {
        let mut buffer = String::new();
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            ok = false;
            continue;
        }
        eprintln!("plugin: received \"{}\"", buffer.trim());
        let credentials: Credentials = serde_json::from_str(buffer.as_str())?;

        let status = Status {
            ok: authenticator.check(&credentials.username, &credentials.password),
        };
        let text = serde_json::to_string(&status)?;

        eprintln!("plugin: sending \"{}\"", text);
        io::stdout().write_all(text.as_bytes())?;
        io::stdout().write_all(b"\n")?;
    }

    eprintln!("plugin: exiting");

    Ok(())
}

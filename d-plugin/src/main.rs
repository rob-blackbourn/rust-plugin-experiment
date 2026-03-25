use std::io::{self, Write};

use d_common::{Credentials, Status};

mod args;
use args::Args;

mod authenticator;
use authenticator::HtpasswdAuthenticator;

fn main() -> io::Result<()> {
    let args = Args::load()?;
    let authenticator = HtpasswdAuthenticator::new(&args.password_file)?;

    loop {
        let Some(credentials) = read_credentials()? else {
            break;
        };

        let ok = authenticator.check(&credentials.username, &credentials.password);

        write_status(Status { ok })?;
    }

    Ok(())
}

fn read_credentials() -> io::Result<Option<Credentials>> {
    let mut buffer = String::new();
    let bytes_read = io::stdin().read_line(&mut buffer)?;
    if bytes_read == 0 {
        return Ok(None);
    }
    let credentials: Credentials = serde_json::from_str(&buffer)?;
    Ok(Some(credentials))
}

fn write_status(status: Status) -> io::Result<()> {
    let text = serde_json::to_string(&status)?;
    io::stdout().write_all(text.as_bytes())?;
    io::stdout().write_all(b"\n")?;
    Ok(())
}

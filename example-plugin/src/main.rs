use std::io;

fn main() -> io::Result<()> {
    eprintln!("starting plugin");

    let mut buffer = String::new();
    let mut ok = true;
    while ok {
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            ok = false;
            continue;
        }
        let line = buffer.trim();
        if bytes_read == 0 || line == "EXIT" {
            ok = false;
            continue;
        }
        eprintln!("received and echoing \"{}\"", line);
        println!("{}", line);
    }

    eprintln!("plugin exiting");

    Ok(())
}

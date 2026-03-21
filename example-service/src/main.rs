use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut ok = true;
    while ok {
        println!("Type \"EXIT\" to quit");
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 || buffer.trim() == "EXIT" {
            ok = false;
        }
        print!("{}", buffer);
    }

    println!("Existed normally");

    Ok(())
}

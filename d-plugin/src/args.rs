use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Args {
    pub password_file: PathBuf,
}

fn fetch_arg(arg_name: &str, args: &[String], arg_index: &mut usize) -> io::Result<String> {
    *arg_index = *arg_index + 1;
    if *arg_index >= args.len() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("insufficient arguments for {}", arg_name),
        ));
    }
    let arg = args.get(*arg_index).unwrap();

    Ok(arg.clone())
}

fn check_fetch_arg<T>(
    arg_name: &str,
    current_value: &Option<T>,
    args: &[String],
    arg_index: &mut usize,
) -> io::Result<String> {
    if current_value.is_some() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("argument {} requires a parameter", arg_name),
        ));
    }

    fetch_arg(arg_name, args, arg_index)
}

impl Args {
    pub fn parse(args: &[String]) -> io::Result<Self> {
        let mut password_file: Option<PathBuf> = None;

        let mut arg_index = 1;
        while arg_index < args.len() {
            let arg_name = args.get(arg_index).unwrap().as_str();
            match arg_name {
                "--password-file" => {
                    let arg = check_fetch_arg(arg_name, &password_file, &args, &mut arg_index)?;
                    password_file = Some(PathBuf::from(arg));
                }
                "--help" => Err(io::Error::new(
                    io::ErrorKind::Other,
                    Self::usage(args.get(0).unwrap()),
                ))?,
                _ => Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("invalid argument {}", arg_name),
                ))?,
            }

            arg_index += 1
        }

        let Some(password_file) = password_file else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                Self::usage(args.get(0).unwrap()),
            ));
        };

        return Ok(Self { password_file });
    }

    pub fn usage(prog_name: &str) -> String {
        format!(
            "usage:
            \t{prog_name} [<options>]
            
            options:
            \t--password-file <path>
            "
        )
    }

    pub fn load() -> io::Result<Self> {
        let args: Vec<String> = std::env::args().collect();
        match Self::parse(&args) {
            Ok(args) => Ok(args),
            Err(error) => {
                let prog_name = args.get(0).unwrap();
                let s = Self::usage(&prog_name);
                println!("error: {error}\n{s}");
                Err(error)
            }
        }
    }
}

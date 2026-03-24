use std::env;
use std::io;

pub struct Args {
    pub plugin_path: String,
    pub plugin_cmdline: Vec<String>,
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

fn current_exe_dir<'a>() -> String {
    let mut path = env::current_exe().expect("should find exe dir");
    path.pop();
    String::from(path.to_string_lossy())
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
        let mut plugin_path: Option<String> = None;
        let mut plugin_cmdline: Option<Vec<String>> = None;

        let mut arg_index = 1;
        while arg_index < args.len() {
            let arg_name = args.get(arg_index).unwrap().as_str();
            match arg_name {
                "--plugin-path" => {
                    let arg = check_fetch_arg(arg_name, &plugin_path, &args, &mut arg_index)?;
                    plugin_path = Some(arg);
                }
                "--plugin-cmdline" => {
                    let arg = check_fetch_arg(arg_name, &plugin_cmdline, &args, &mut arg_index)?;
                    match shell_words::split(arg.as_str()) {
                        Ok(cmdline) => plugin_cmdline = Some(cmdline),
                        Err(_) => Err(io::Error::new(
                            io::ErrorKind::Other,
                            "invalid plugin cmdline",
                        ))?,
                    }
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

        let plugin_path = plugin_path.or(Some(current_exe_dir())).unwrap();
        let plugin_cmdline = plugin_cmdline.or(Some(Vec::new())).unwrap();

        return Ok(Self {
            plugin_path,
            plugin_cmdline,
        });
    }

    pub fn usage(prog_name: &str) -> String {
        format!(
            "usage:
            \t{prog_name} [<options>]
            
            options:
            \t--plugin-path <path> # defaults to \".\"
            \t--plugin-cmdline <cmdline> # defaults to \"\"
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

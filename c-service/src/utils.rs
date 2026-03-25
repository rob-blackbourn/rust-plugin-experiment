use std::env;
use std::io;
use std::path::Path;

pub fn read_stdin() -> io::Result<Option<String>> {
    let mut line = String::new();
    let bytes_read = io::stdin().read_line(&mut line)?;
    if bytes_read == 0 {
        Ok(None)
    } else {
        Ok(Some(line))
    }
}

pub fn add_plugin_path(path: &str) -> () {
    let key = "PATH";
    match env::var_os(key) {
        Some(path_var) => {
            let mut paths = env::split_paths(&path_var).collect::<Vec<_>>();
            paths.push(path.into());
            let new_path = env::join_paths(paths).expect("should join paths");
            unsafe {
                env::set_var(key, &new_path);
            }
        }
        None => {
            let paths = vec![Path::new(path)];
            let path_var = env::join_paths(paths.iter()).expect("should join path");
            unsafe {
                env::set_var(key, &path_var);
            }
        }
    }
}

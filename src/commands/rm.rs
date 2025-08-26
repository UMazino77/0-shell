use crate::zero::*;
use std::fs;
use std::io;
use std::path::Path;

pub fn exec_rm(
    cmd: Commands,
    args: &[String],
    mp: &mut std::collections::HashMap<Commands, String>
) -> io::Result<()> {
    detect_flags(cmd.clone(), args, mp);

    for i in args {
        if i.starts_with('-') {
            continue;
        }

        let path_str = format!("./{}", i);
        let path = Path::new(&path_str);

        if !path.exists() {
            println!("rm: cannot remove '{}': No such file or directory", i);
            continue;
        }

        if !valid_flags(cmd.clone(), mp) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid flags"));
        }

        let metadata = fs::metadata(path)?;

        if metadata.is_dir() {
            if mp.contains_key(&cmd) && mp.get(&cmd) == Some(&"r".to_string()) {
                fs::remove_dir_all(path)?;
            } else {
                println!("rm: cannot remove '{}': Is a directory", i);
            }
        } else if metadata.is_file() {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

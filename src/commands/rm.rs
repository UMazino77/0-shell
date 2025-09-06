use crate::zero::*;
use std::fs;
use std::io;
use std::path::Path;

pub fn exec_rm(
    cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) -> io::Result<()> {
    detect_flags(cmd.clone(), args, mp);

    for i in args {
        let path_str = format!("./{}", i);

        let path = Path::new(&path_str);

        if path.symlink_metadata().is_err() {
            println!("rm: cannot remove '{}': No such file or directory", i);
            continue;
        }

        if !valid_flags(cmd.clone(), mp) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid flags"));
        }

        let metadata = path.symlink_metadata()?;

        if metadata.is_dir() {
            // println!("{:?}  ++++++   {:?}", metadata.file_type(), mp);
            if mp.contains_key(&cmd) && mp.get(&cmd) == Some(&"r".to_string()) {
                fs::remove_dir_all(path)?;
            } else {
                println!("rm: cannot remove '{}': Is a directory", i);
            }
        } else {
            fs::remove_file(path)?;
        }
    }
    
    Ok(())
}
use crate::commands::ls::create_path;
use crate::zero::*;
use std::fs;
use std::io;

pub fn exec_rm(
    cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) -> io::Result<()> {
    detect_flags(cmd.clone(), args, mp);
    if valid_flags(cmd.clone(), mp) == false {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid flags"));
    }

    for i in args {

        // if i == "." || i == ".." {
        //     println!("rm: refusing to remove '.' or '..' directory: skipping '..'");
        //     continue;
        // }

        let path = create_path(String::from(".") ,i.clone());

        if path.symlink_metadata().is_err() {
            println!("rm: cannot remove '{}': No such file or directory", i);
            continue;
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
use std::fs;
use crate::{commands::ls::create_path, zero::*};
use crate::commands::cp::*;



pub fn exec_mv(
    cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    detect_flags(cmd.clone(), args, mp);

    if args.len() < 2 {
        eprintln!("Usage: mv SOURCE... DEST");
        return;
    }

    if !valid_flags(cmd.clone(), mp) {
        return;
    }

    let dest = &args[args.len() - 1];
    let srcs = &args[0..args.len() - 1];
    let dest_path = create_path(".".to_owned(), dest.to_string());

    if srcs.len() > 1 && (!dest_path.exists() || !dest_path.is_dir()) {
        eprintln!("mv: target '{}' is not a directory", dest);
        return;
    }

    for src in srcs {
        let src_path = create_path(".".to_owned(), src.to_string());

        if !src_path.exists() {
            eprintln!("mv: cannot stat '{}': No such file or directory", src);
            continue;
        }

        let final_dest = if dest_path.exists() && dest_path.is_dir() {
            dest_path.join(src_path.file_name().unwrap())
        } else {
            dest_path.to_path_buf()
        };

        if paths_equal(&src_path, &final_dest) {
            eprintln!("mv: '{}' and '{}' are the same file", src, final_dest.display());
            continue;
        }

        if src_path.is_dir() && is_inside(&src_path, &final_dest) {
            eprintln!("mv: cannot move a directory, '{}', into itself, '{}'", src, final_dest.display());
            continue;
        }

        if src_path.is_dir() && final_dest.exists() && !final_dest.is_dir() {
            eprintln!("mv: cannot overwrite non-directory '{}' with directory '{}'", final_dest.display(), src);
            continue;
        }

        match fs::rename(&src_path, &final_dest) {
            Ok(_) => {
            },
            Err(_) => {
                let final_dest_str = final_dest.to_string_lossy().to_string();
                
                let mut copy_args = vec![src.clone(), final_dest_str.clone()];
                if src_path.is_dir() {
                    copy_args.push(String::from("-r"));
                }
                execute(Commands::Cp, &mut copy_args, &mut mp.clone());
                
                if final_dest.exists() {
                    let mut rm_args = vec![src.clone()];
                    if src_path.is_dir() {
                        rm_args.push(String::from("-r"));
                    }
                    execute(Commands::Rm, &mut rm_args, &mut mp.clone());
                } else {
                    eprintln!("mv: cannot move '{}': copy operation failed", src);
                }
            }
        }
    }
}

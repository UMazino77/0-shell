use crate::zero::Commands;
use crate::zero::*;
use std::fs::*;
use crate::commands::ls::create_path;

pub fn exec_cp(
    cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    detect_flags(cmd.clone(), args, mp);

    if args.len() < 2 {
        eprintln!("Usage: cp SOURCE DEST");
        return;
    }

    if !valid_flags(cmd.clone(), mp) {
        return;
    }

    let src = &args[0];
    let dest = &args[1];

    let src_path = create_path(".".to_owned(), src.to_string());
    let dest_path = create_path(".".to_owned(), dest.to_string());

    if !src_path.exists() {
        eprintln!("cp: cannot stat '{}': No such file or directory", src);
        return;
    }

        let final_dest = if dest_path.exists() && dest_path.is_dir() {
                dest_path.join(src_path.file_name().unwrap())
            } else {
                dest_path.to_path_buf()
            };

    if src_path.is_dir() {
        if mp.contains_key(&Commands::Cp) && mp.get(&Commands::Cp) == Some(&"r".to_string()) {
            if let Err(e) = create_dir_all(&final_dest) {
                eprintln!("cp: error creating directory '{}': {}", final_dest.display(), e);
                return;
            }

            for i in read_dir(src_path).unwrap() {
                let ii = i.unwrap();
                let file_name = ii.file_name();
                let new_dest = final_dest.join(file_name);
                exec_cp(
                    cmd.clone(),
                    &mut vec![ii.path().to_str().unwrap().to_string(), new_dest.to_str().unwrap().to_string()],
                    mp
                );
            }
        } else {
            eprintln!("cp: -r not specified; omitting directory '{}'", src);
            return;
        }
    } else {
        if let Err(e) = copy(src_path, &final_dest) {
            eprintln!("cp: error copying file '{}': {}", src, e);
        }
    }
}
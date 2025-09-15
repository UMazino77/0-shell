use crate::zero::Commands;
use crate::zero::*;
use std::fs::*;
use crate::commands::ls::create_path;
use std::path::Path;

pub fn exec_cp(
    cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    detect_flags(cmd.clone(), args, mp);

    if args.len() < 2 {
        eprintln!("Usage: cp SOURCE... DEST");
        return;
    }

    if !valid_flags(cmd.clone(), mp) {
        return;
    }

    let dest = &args[args.len() - 1];
    let srcs = &args[0..args.len() - 1];
    let dest_path = create_path(".".to_owned(), dest.to_string());
    let rec = mp.contains_key(&Commands::Cp) && mp.get(&Commands::Cp) == Some(&"r".to_string());

    if srcs.len() > 1 && (!dest_path.exists() || !dest_path.is_dir()) {
        eprintln!("cp: target '{}' is not a directory", dest);
        return;
    }

    for src in srcs {
        let src_path = create_path(".".to_owned(), src.to_string());

        if !src_path.exists() {
            eprintln!("cp: cannot stat '{}': No such file or directory", src);
            continue;
        }

        let final_dest = if dest_path.exists() && dest_path.is_dir() {
            dest_path.join(src_path.file_name().unwrap())
        } else {
            dest_path.to_path_buf()
        };

        if paths_equal(&src_path, &final_dest) {
            eprintln!("cp: '{}' and '{}' are the same file", src, final_dest.display());
            continue;
        }

        if src_path.is_dir() && is_inside(&src_path, &final_dest) {
            eprintln!("cp: cannot copy a directory, '{}', into itself, '{}'", src, final_dest.display());
            continue;
        }

        if src_path.is_dir() {
            if rec {
                copy_recursive(&src_path, &final_dest);
            } else {
                eprintln!("cp: -r not specified; omitting directory '{}'", src);
            }
        } else {
            if let Err(e) = copy(&src_path, &final_dest) {
                eprintln!("cp: error copying file '{}': {}", src, e);
            }
        }
    }
}

pub fn paths_equal(path1: &Path, path2: &Path) -> bool {
    match (path1.canonicalize(), path2.canonicalize()) {
        (Ok(p1), Ok(p2)) => p1 == p2,
        _ => false,
    }
}

pub fn is_inside(src: &Path, dest: &Path) -> bool {
    if let Ok(src_canonical) = src.canonicalize() {
        if let Ok(dest_canonical) = dest.canonicalize() {
            return dest_canonical.starts_with(&src_canonical);
        }
        
        let mut current = dest;
        while let Some(parent) = current.parent() {
            if paths_equal(&src_canonical, parent) {
                return true;
            }
            current = parent;
        }
    }
    false
}

pub fn copy_recursive(src: &Path, dest: &Path) {
    if let Err(e) = create_dir_all(dest) {
        eprintln!("cp: error creating directory '{}': {}", dest.display(), e);
        return;
    }

    if let Ok(entries) = read_dir(src) {
        for entry in entries.flatten() {
            let src_item = entry.path();
            let dest_item = dest.join(entry.file_name());

            if src_item.is_dir() {
                copy_recursive(&src_item, &dest_item);
            } else {
                if let Err(e) = copy(&src_item, &dest_item) {
                    eprintln!("cp: error copying file '{}': {}", src_item.display(), e);
                }
            }
        }
    }
}
use std::fs;
use crate::zero::*;
use std::path::Path;
use crate::commands::cp::exec_cp;
use crate::commands::rm::exec_rm;

pub fn exec_mv(
    cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    detect_flags(cmd.clone(), args, mp);

    if args.len() < 2 {
        println!("mv: missing file operand");
        return;
    }

    let src = &args[0];
    let dest = &args[1];

    let src_path_str = format!("./{}", src);
    let dest_path_str = format!("./{}", dest);

    let src_path = Path::new(&src_path_str);
    let dest_path = Path::new(&dest_path_str);

    if !src_path.exists() {
        println!("mv: cannot stat '{}': No such file or directory", src);
        return;
    }

    if !valid_flags(cmd.clone(), mp) {
        println!("mv: invalid flags");
        return;
    }

    if dest_path.exists() {
        if dest_path.is_dir() {
            exec_cp(Commands::Cp, &mut vec![src.clone(), dest.clone(), String::from("-r")], mp);
            if let Err(e) = exec_rm(Commands::Rm, &mut vec![src.clone(), String::from("-r")], mp) {
                println!("mv: cannot remove '{}': {}", src, e);
            }
        } else {
            if src_path.is_dir() {
                println!("mv: cannot overwrite non-directory '{}' with directory '{}'", dest, src);
                return;
            }
            if let Err(e) = fs::rename(src_path, dest_path) {
                println!("mv: cannot move '{}': {}", src, e);
            }
        }
    } else {
        if let Err(e) = fs::rename(src_path, dest_path) {
            println!("mv: cannot move '{}': {}", src, e);
        }
    }
}

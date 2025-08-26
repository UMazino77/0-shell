use crate::zero::*;
use crate::zero::Commands::Mkdir;
use std::path::Path;

pub fn exec_mkdir(
    _cmd: crate::zero::Commands,
    args: &mut [String],
    mp: &mut std::collections::HashMap<crate::zero::Commands, String>
) {
    if args.len() < 1 {
        println!("mkdir: missing operand");
        return;
    }

    detect_flags(Mkdir, args, mp);

    if args.len() > 1 && !mp.contains_key(&Mkdir) {
        println!("mkdir: too many arguments");
        return;
    }
    if mp.contains_key(&Mkdir) && mp.get(&Mkdir) == Some(&"p".to_string()) {
        for dir in args.iter() {
            if !dir.starts_with('-') {
                let path = Path::new(dir);
                if path.exists() {
                    eprintln!("mkdir: cannot create directory '{}': File exists", dir);
                    continue;
                }
                if let Err(_) = std::fs::create_dir_all(dir) {
                    return ;
                }
            }
        }
        return;
    }
    let dir_name = &args[0];
    let path = Path::new(dir_name);
    if path.exists() {
        eprintln!("mkdir: cannot create directory '{}': File exists", dir_name);
    }
    if let Err(_) = std::fs::create_dir(dir_name) {
        return ;
    }
}

use crate::zero::Commands::Mkdir;
use crate::zero::*;
use std::path::Path;

pub fn exec_mkdir(
    _cmd: crate::zero::Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<crate::zero::Commands, String>,
) {
    if args.len() < 1 {
        println!("mkdir: missing operand");
        return;
    }

    detect_flags(Mkdir, args, mp);
    if !valid_flags(Mkdir, mp) {
        // println!("mkdir: invalid option");
        return;
    }

    // println!("{:?}", args);

    for dir in args.iter() {
        let path = Path::new(dir);
        if path.exists() {
            eprintln!("mkdir: cannot create directory '{}': File exists", dir);
            continue;
        }
        if let Err(_) = std::fs::create_dir(dir) {
            continue;
        }
    }
}

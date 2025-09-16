use crate::zero::{detect_flags, valid_flags};
use std::fs::* ;

pub fn exec_touch(
    cmd: crate::zero::Commands,
    args: &mut Vec<String>,
    cmd_map: &mut std::collections::HashMap<crate::zero::Commands, String>
) {
    if args.is_empty() {
        println!("touch: missing file operand");
        return;
    }

    detect_flags(cmd.clone(), args, cmd_map);
    if !valid_flags(cmd, cmd_map) {
        return;
    }

    for filename in args.iter() {
        match OpenOptions::new()
            .create(true)
            .write(true)
            .open(filename)
        {
            Ok(_) => {}
            Err(e) => {
                println!("touch: cannot touch '{}': {}", filename, e);
            }
        }
    }
}
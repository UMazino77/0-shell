use crate::zero::{detect_flags, valid_flags};
use std::fs::* ;

pub fn exec_touch(
    _cmd: crate::zero::Commands,
    args: &mut Vec<String>,
    _mp: &mut std::collections::HashMap<crate::zero::Commands, String>
) {
    if args.is_empty() {
        println!("touch: missing file operand");
        return;
    }

    detect_flags(_cmd.clone(), args, _mp);
    if !valid_flags(_cmd, _mp) {
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
use crate::zero::*;
use crate::zero::Commands;

pub fn exec_ls(
    cmd: Commands,
    _args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    detect_flags(cmd.clone(), _args, mp);
    if !valid_flags(cmd.clone(), mp) {
        return;
    }

    match mp.get(&cmd) {
        Some(flags) => {
            if flags.contains('a') {
                println!("all");
            }
            if flags.contains('F') {
                println!("classify");
            }
            if flags.contains('l') {
                println!("long format");
            }
        }
        None => {
            // default_ls();
            println!("default");
        }
    }
    mp.clear();
}

// pub fn default_ls() {

// }
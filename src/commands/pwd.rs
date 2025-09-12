use std::env::current_dir;
use crate::zero::*;

pub fn exec_pwd(
    _cmd: Commands,
    _args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    let current_dir = current_dir();
    if let Ok(current_dir) = current_dir {
        println!("{}", current_dir.display());
    } else {
        let a = mp.get(&Commands::Pwd);
        eprintln!("{}", a.unwrap_or(&"Unknown error".to_string()));
    }
}

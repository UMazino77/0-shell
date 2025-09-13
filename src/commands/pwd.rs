use std::env::current_dir;
use crate::zero::*;

pub fn exec_pwd(
    _cmd: Commands,
    _args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    let current_dir = current_dir();
    let curr = mp.get(&Commands::Pwd);
    if curr.is_none() {
        if let Ok(path) = current_dir {
            let path_str = path.to_string_lossy().to_string();
            println!("{}", path_str);
            mp.insert(Commands::Pwd, path_str);
        } else {
            println!("pwd: error retrieving current directory");
        }
    } else {
        println!("{}", curr.unwrap());
    }
    
}

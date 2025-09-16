use std::env::current_dir;
use crate::zero::*;

pub fn exec_pwd(
    cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    detect_flags(cmd.clone(), args, mp);
    if valid_flags(cmd.clone(), mp) == false {
        return ;
    }
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

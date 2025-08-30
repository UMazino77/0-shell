use crate::zero::Commands;
use crate::zero::*;
use crate::commands::rm::exec_rm;
use crate::commands::cat::exec_cat;



pub fn exec_history(
    cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    detect_flags(cmd.clone(), args, mp);
    if !valid_flags(cmd.clone(), mp) {
        println!("history: invalid flags");
        return;
    }
    if args.len() > 0 {
        println!("history: too many arguments");
        return;
    }
    let user = whoami::username();
    let his_path = format!("/home/{}/.zero-history.txt", user) ;
    if mp.contains_key(&cmd) && mp.get(&cmd) == Some(&"c".to_string()) {
        let _ = exec_rm(cmd.clone(), &mut vec![his_path.to_string()], mp);
        mp.remove(&cmd);
    } else {
        exec_cat(Commands::Cat, &mut vec![his_path.to_string(), "-n".to_string()], mp);
    }
    
}
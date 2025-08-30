use crate::zero::Commands;
use std::env::*;

pub fn exec_cd(
    _cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    {
        let user = whoami::username();

        if args.len() < 1 {
            *args = vec![String::from("~")];
        }
        if args.len() > 1 {
            println!("cd: too many arguments");
            return;
        }

        let mut path = &mut args[0].replace("~", &format!("/home/{}", user));

        // println!("{}", path);

        if path == "-" {
            if mp.contains_key(&Commands::Cd) && let Some(old_path) = mp.get(&Commands::Cd) {
                println!("{}", old_path);
                if let Err(e) = set_current_dir(old_path) {
                    println!("cd: {}: {}", old_path.replace(&format!("/home/{}", user), "~"), e);
                } else {
                    mp.insert(
                        Commands::Cd,
                        std::env::current_dir().unwrap().to_str().unwrap().to_string()
                    );
                }
            }
            return;
        }
        mp.insert(Commands::Cd, current_dir().unwrap().to_str().unwrap().to_string());
        if let Err(e) = set_current_dir(&mut path) {
            println!("cd: {}: {}", path, e);
        }
        // println!("{}", current_dir().unwrap().to_str().unwrap());
    }
}

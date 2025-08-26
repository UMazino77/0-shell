use crate::zero::Commands;

pub fn exec_cd(
    _cmd: crate::zero::Commands,
    args: &mut [String],
    mp: &mut std::collections::HashMap<crate::zero::Commands, String>
) {
    {
        if args.len() < 1 {
            println!("cd: missing operand");
        } else {
            if args.len() > 1 {
                println!("cd: too many arguments");
                return;
            }
            let mut path = &mut args[0];
            let user = whoami::username();
            *path = path.replace("~", &("/home/".to_owned() + &user));
            if path == "-" {
                if mp.contains_key(&Commands::Cd) && let Some(old_path) = mp.get(&Commands::Cd) {
                    println!("{}", old_path);
                    if let Err(e) = std::env::set_current_dir(old_path) {
                        println!("cd: {}: {}", old_path, e);
                    } else {
                        mp.insert(
                            Commands::Cd,
                            std::env::current_dir().unwrap().to_str().unwrap().to_string()
                        );
                    }
                }
                return;
            }
            mp.insert(Commands::Cd, std::env::current_dir().unwrap().to_str().unwrap().to_string());
            if let Err(e) = std::env::set_current_dir(&mut path) {
                println!("cd: {}: {}", path, e);
            }
            // println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
        }
    }
}

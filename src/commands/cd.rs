use crate::zero::Commands;
use std::env::*;
use std::path::Path;

pub fn exec_cd(
    _cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    let user = whoami::username();

    if args.is_empty() {
        *args = vec![String::from("~")];
    }

    if args.len() > 1 {
        println!("cd: too many arguments");
        return;
    }

    if args[0] == "--" {
        args[0] = String::from("~");
    }

    let target_path = args[0].replace("~", &format!("/home/{}", user));

    if target_path == "-" {
        match mp.get(&Commands::Cd) {
            Some(old_path) => {
                if !Path::new(old_path).exists() {
                    println!(
                        "cd: {}: No such file or directory",
                        old_path.replace(&format!("/home/{}", user), "~")
                    );
                    return;
                }
                let current_before = get_current_dir();
                println!("{}", old_path.replace(&format!("/home/{}", user), "~"));
                if let Err(e) = set_current_dir(old_path) {
                    println!("cd: {}: {}", old_path.replace(&format!("/home/{}", user), "~"), e);
                } else if let Some(prev) = current_before {
                    mp.insert(Commands::Cd, prev);
                }
            }
            None => println!("cd: OLDPWD not set"),
        }
        return;
    }

    if !Path::new(&target_path).exists() {
        println!(
            "cd: {}: No such file or directory",
            target_path.replace(&format!("/home/{}", user), "~")
        );
        return;
    }

    if !Path::new(&target_path).is_dir() {
        println!("cd: {}: Not a directory", target_path.replace(&format!("/home/{}", user), "~"));
        return;
    }

    let current_before = get_current_dir();
    if let Err(e) = set_current_dir(&target_path) {
        println!("cd: {}: {}", target_path.replace(&format!("/home/{}", user), "~"), e);
        return;
    } else if let Some(prev) = current_before {
        mp.insert(Commands::Cd, prev);
    }

    let currr = get_current_dir();
    
    mp.insert(Commands::Pwd, currr.unwrap_or("Unkno".to_string()));

}

fn get_current_dir() -> Option<String> {
    current_dir()
        .ok()
        .and_then(|p| {
            let path_str = p.to_string_lossy().to_string();
            if Path::new(&path_str).exists() {
                Some(path_str)
            } else {
                None
            }
        })
}

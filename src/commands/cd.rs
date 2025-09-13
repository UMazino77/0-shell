use crate::commands::ls::create_path;
use crate::zero::Commands;
use std::{env::*, path};
use std::fs::*;
use std::path::*;

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
            None => _ = {mp.insert(Commands::Cd, get_current_dir().unwrap_or("~".into())); set_current_dir(&format!("/home/{}", user))},
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

    println!("{}", target_path);

   let Ok(link) = create_path(String::from("."), target_path.clone()).symlink_metadata() else {
        println!("ddddd");
        mp.insert(Commands::Pwd, currr.unwrap_or("Unkno".to_string()));
        return;
    };

    if link.file_type().is_symlink() {
        println!("{}", target_path);
        mp.insert(Commands::Pwd, target_path);
        return;
    }
    
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

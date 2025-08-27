use crate::zero::*;
use crate::zero::Commands;
use std::path::Path;

pub fn exec_ls(
    cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    detect_flags(cmd.clone(), args, mp);
    if !valid_flags(cmd.clone(), mp) {
        return;
    }

    let mut files = Vec::new();
    let mut folders = Vec::new();

    handle_files_folders(&mut files, &mut folders, args);

    match mp.get(&cmd) {
        Some(flags) => {
            if flags.contains('a') {
                folders.insert(0, String::from(".."));
                folders.insert(0, String::from("."));
            }
            if flags.contains('l') {
                println!("long format");
            }
        }
        None => {
            default_ls(files.clone(), folders.clone());
            // println!("default");
        }
    }

    println!();
    println!();
    println!();
    println!("{:?}  --- ++++ ", folders);
    println!();
    println!();
    println!();
    println!();
    println!("{:?}  --- ++++ ", files);

}

pub fn handle_files_folders(
    files: &mut Vec<String>,
    folders: &mut Vec<String>,
    args: &mut Vec<String>
) {
    for i in args {
        let fd_name = format!("./{}", i);
        let path = Path::new(&fd_name);
        if !path.exists() {
            println!("ls: cannot access '{}': No such file or directory", i);
            continue;
        }
        if path.is_dir() {
            folders.push(i.clone());
        } else {
            files.push(i.clone());
        }
    }
}

pub fn default_ls(files : Vec<String> , _folders : Vec<String>) {
    display_files(files) ;
    // display_folders(folders) ;
}

pub fn display_files(files : Vec<String>) {
    // let ter_width = todo!() ;
    // let max = ter_width
    let mut j = 0 ; 
    for i in &files {
        if j == files.len()-1 {
            println!("{i}");
        } else {
            print!("{}  ", i.clone()) ;
        }
        j += 1 ;
    }
}
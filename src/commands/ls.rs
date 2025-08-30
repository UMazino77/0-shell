use crate::zero::*;
use crate::zero::Commands;
use std::fs::*;
use std::os::unix::fs::*;
use std::path::PathBuf;

pub fn exec_ls(
    cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    detect_flags(cmd.clone(), args, mp);
    if !valid_flags(cmd.clone(), mp) {
        return;
    }

    if args.is_empty() {
        args.push(String::from("."));
    }
    let mut hidden = false;
    let mut files = Vec::new();
    let mut folders = Vec::new();
    let mut dash = false ;

    handle_files_folders(&mut files, &mut folders, args);

    sort_fd(&mut files);
    sort_fd(&mut folders);

    // println!("{:?}", folders);

    match mp.get(&cmd) {
        Some(flags) => {
            if flags.contains('a') {
                hidden = true;
            }
            if flags.contains('F') {
                dash = true;
            }
            if flags.contains('l') {
                long_ls(files.clone(), folders.clone(), hidden);
            } else {
                default_ls(files.clone(), folders.clone(), hidden, dash);
            }
        }
        None => {
            default_ls(files.clone(), folders.clone(), hidden, dash);
            // println!("default");
        }
    }
}

pub fn handle_files_folders(
    files: &mut Vec<String>,
    folders: &mut Vec<String>,
    args: &mut Vec<String>
) {
    for i in args {

        let path = crate_path(i.clone());
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

pub fn default_ls(files: Vec<String>, folders: Vec<String>, hidden: bool, dash : bool) {
    display_files(String::from("."), files.clone(), folders.len() > 1, dash);
    display_folders(folders.clone(), files.len() != 0 || folders.len() > 1, hidden, dash);
}

pub fn long_ls(files: Vec<String>, folders: Vec<String>, _hidden: bool) {
    display_long_files(files.clone(), folders.len() > 1);
    // display_long_folders(folders.clone(), files.len() != 0 || folders.len() > 1, hidden);
}

pub fn display_files(parent : String ,files: Vec<String>, cc: bool, dash: bool) {
    // let ter_width = todo!() ;
    // let max = ter_width
    let mut j = 0;
    for i in &files {

        let path = crate_path(format!("{}/{}",parent, i));
        let ff = dash_f(path, dash) ;

        if j == files.len() - 1 {
            println!("{i}{ff}");
            if cc {
                println!();
            }
        } else {
            print!("{}{ff}  ", i.clone());
        }
        j += 1;
    }
}

pub fn display_folders(folders: Vec<String>, cc: bool, hidden: bool, dash : bool) {
    let mut jj = 0;
    for i in &folders {
        let a = crate_path(i.clone());
        let mut aa: Vec<_> = read_dir(a).unwrap().collect();

        if cc {
            println!("{i}:");
        }

        if !hidden {
            aa.retain(|x| !x.as_ref().unwrap().file_name().to_string_lossy().starts_with("."));
        }

        let mut new_fold = vec![];

        for i in &aa {
            let ii = i.as_ref().unwrap();
            let name = ii.file_name();
            new_fold.push(name.to_string_lossy().to_string());
        }

        if hidden {
            new_fold.insert(0, String::from(".."));
            new_fold.insert(0, String::from("."));
        }
        // println!("{:?}", new_fold) ;
        sort_fd(&mut new_fold);
        // println!("{:?}  +++", new_fold) ;

        display_files(i.to_string(),new_fold, jj != folders.clone().len() - 1, dash);

        jj += 1;
    }
}

pub fn sort_fd(a: &mut Vec<String>) {
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            let aa: Vec<_> = a[i]
                .clone()
                .to_ascii_lowercase()
                .chars()
                .into_iter()
                .filter(|x| x.is_alphanumeric())
                .collect();
            let bb: Vec<_> = a[j]
                .clone()
                .to_ascii_lowercase()
                .chars()
                .into_iter()
                .filter(|x| x.is_alphanumeric())
                .collect();
            // if aa == bb {
            //     /*
            //         check by time of last modification
            //     */
            // }
            for k in 0..min(aa.len(), bb.len()) {
                if aa[k] > bb[k] || (k == min(aa.len(), bb.len()) - 1 && aa.len() > bb.len()) {
                    let temp = a[i].clone();
                    a[i] = a[j].clone();
                    a[j] = temp;
                    break;
                } else if aa[k] < bb[k] {
                    break;
                }
            }
        }
    }
}

pub fn crate_path(a: String) -> PathBuf {
    if a.starts_with("/") { PathBuf::from(a) } else { PathBuf::from(format!("./{}", a)) }
}

pub fn min(a: usize, b: usize) -> usize {
    if a < b {
        return a;
    }
    b
}


pub fn dash_f(path: PathBuf, dash: bool) -> String {
    if !dash {
        return String::new();
    }
    
    let Ok(metadata) = path.symlink_metadata() else {
        return String::from("?");
    };
    
    let file_type = metadata.file_type();
    
    if file_type.is_dir() {
        return String::from("/");
    }
    
    if file_type.is_symlink() {
        return String::from("@");
    }
    
    if file_type.is_file() {
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        
        if mode & 0o111 != 0 {
            return String::from("*");
        }
        
        return String::new();
    }
    
    if file_type.is_fifo() {
        return String::from("|");
    }
    
    if file_type.is_socket() {
        return String::from("=");
    }
    String::new()
}

pub fn display_long_files(files: Vec<String>, cc: bool) {
    for (index, file) in files.iter().enumerate() {
        let path = crate_path(file.clone());

        
        if let Ok(metadata) = path.metadata() {
            let perms = metadata.mode();
            println!("{}", perms);
        }

        println!("++++++");

        if index == files.len() - 1 && cc {
            println!();
        } else {
            println!("{}  ", file);
        }
    }
}


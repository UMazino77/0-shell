use crate::zero::*;
use crate::zero::Commands;
use std::fs::*;
use std::os::unix::fs::*;
use std::path::*;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use users::{get_user_by_uid, get_group_by_gid};

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
                long_ls(files.clone(), folders.clone(), hidden,dash);
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

        let path = create_path(i.clone());
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
    display_files(String::from("."), files.clone(), folders.len() > 0, dash);
    display_folders(folders.clone(), files.len() != 0 || folders.len() > 1, hidden, dash);
}

pub fn long_ls(files: Vec<String>, folders: Vec<String>, hidden: bool,dash : bool) {
    display_long_files(String::from("."),files.clone(), folders.len() > 0,dash);
    display_long_folders(folders.clone(), files.len() != 0 || folders.len() > 1, hidden, dash);
}

pub fn display_files(parent : String ,files: Vec<String>, cc: bool, dash: bool) {
    // let ter_width = todo!() ;
    // let max = ter_width
    let mut j = 0;
    for i in &files {

        let path = create_path(format!("{}/{}",parent, i));
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
        let a = create_path(i.clone());
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

pub fn create_path(a: String) -> PathBuf {
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

pub fn display_long_files(parent : String , files: Vec<String>, cc: bool,dash : bool) {
    for (index, file) in files.iter().enumerate() {
        let path = create_path(format!("{}/{}",parent, file));

        let Ok(metadata) = path.symlink_metadata() else {
            continue ;
        };

        let perms = permissions(path.clone()) ;

        let uid = metadata.uid();
        let gid = metadata.gid();

        let links = metadata.nlink();
        // println!("++++++");
        let username = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().to_string())
            .unwrap_or_else(|| uid.to_string());
            
        let groupname = get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().to_string())
            .unwrap_or_else(|| gid.to_string());

        

        let size = metadata.size();

        let d = dash_f(path.clone(), dash) ;

        if let Some(target) = read_link_target(&path) {
            println!("{perms} {links} {username} {groupname} {size} {}{d} -> {target}", file.clone());
        } else {
            println!("{perms} {links} {username} {groupname} {size} {}{d}", file.clone());
        }

        if cc && index == files.len() -1 {
            println!();
        }
    }
}

pub fn read_link_target(path: &PathBuf) -> Option<String> {
    if let Ok(metadata) = path.symlink_metadata() {
        if metadata.file_type().is_symlink() {
            if let Ok(target) = path.read_link() {
                return Some(target.to_string_lossy().to_string());
            }
        }
    }
    None
}

pub fn permissions(path: PathBuf) -> String {
    let Ok(metadata) = path.symlink_metadata() else {
        return String::from("---------");
    };
    
    let mode = metadata.mode();
    let file_type = metadata.file_type();
    
    let type_char = if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_socket() {
        's'
    } else if file_type.is_char_device() {
        'c'
    } else if file_type.is_block_device() {
        'b'
    } else {
        '-'
    };

    let acl = if has_acl(&path) {String::from("+")} else {String::new()} ;
    
    let owner_r = if mode & 0o400 != 0 { 'r' } else { '-' };
    let owner_w = if mode & 0o200 != 0 { 'w' } else { '-' };
    let owner_x = match (mode & 0o100 != 0, mode & 0o4000 != 0) {
        (true, true) => 's',   
        (false, true) => 'S',  
        (true, false) => 'x',  
        (false, false) => '-', 
    };
    
    
    let group_r = if mode & 0o040 != 0 { 'r' } else { '-' };
    let group_w = if mode & 0o020 != 0 { 'w' } else { '-' };
    let group_x = match (mode & 0o010 != 0, mode & 0o2000 != 0) {
        (true, true) => 's',   
        (false, true) => 'S', 
        (true, false) => 'x', 
        (false, false) => '-',
    };
    
    let other_r = if mode & 0o004 != 0 { 'r' } else { '-' };
    let other_w = if mode & 0o002 != 0 { 'w' } else { '-' };
    let other_x = match (mode & 0o001 != 0, mode & 0o1000 != 0) {
        (true, true) => 't',  
        (false, true) => 'T',  
        (true, false) => 'x', 
        (false, false) => '-', 
    };
    
    format!("{}{}{}{}{}{}{}{}{}{}{acl}", 
        type_char, owner_r, owner_w, owner_x, 
        group_r, group_w, group_x, 
        other_r, other_w, other_x)
}

fn has_acl(path: &Path) -> bool {
    let path_cstr = match CString::new(path.as_os_str().as_bytes()) {
        Ok(cstr) => cstr,
        Err(_) => return false,
    };
    
    unsafe {
        let result = libc::getxattr(
            path_cstr.as_ptr(),
            b"system.posix_acl_access\0".as_ptr() as *const i8,
            std::ptr::null_mut(),
            0,
        );
        result > 0
    }
}

pub fn display_long_folders(folders: Vec<String>, cc: bool, hidden: bool, dash : bool) {
    let mut jj = 0;
    for i in &folders {
        let a = create_path(i.clone());

        let Ok(aaa) = read_dir(&a) else {
            continue ;
        };

        let mut aa: Vec<_> = aaa.collect();

        let mut total = 0 ;

        if !hidden {
            aa.retain(|x| !x.as_ref().unwrap().file_name().to_string_lossy().starts_with("."));
            total += total_blocks(&aa) ;
        } else {
            total += total_blocks(&aa);
            if let Ok(current_metadata) = a.metadata() {
                total += current_metadata.blocks()/2;
                println!("sss");
            }
            
            let parent_path = create_path(format!("{}/..", i.clone()));
            if let Ok(parent_metadata) = parent_path.metadata() {
                total += parent_metadata.blocks()/2;
                println!("ttt");
            }
        }

        if cc {
            println!("{i}:");
        }
        println!("total {}", total);

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

        display_long_files(i.to_string(), new_fold, jj != folders.clone().len() - 1,dash);

        jj += 1;
    }
}

pub fn total_blocks(aa: &Vec<Result<DirEntry, std::io::Error>>) -> u64 {
    let mut total = 0u64;
    
    for a in aa {
        if let Ok(i) = a {
            if let Ok(metadata) = i.metadata() {
                total += metadata.blocks();
            }
        }
    }
    
    total/2
}
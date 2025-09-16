use crate::zero::*;
use crate::zero::Commands;
use std::fs::*;
use std::os::unix::fs::*;
use std::path::*;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use users::{ get_user_by_uid, get_group_by_gid };
use chrono::*;
use chrono_tz::Africa::Casablanca;
use std::cmp::Ordering;

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
    let mut dash = false;
    let mut dashl = false;
    let mut rev = false;

    match mp.get(&cmd) {
        Some(flags) => {
            if flags.contains('a') {
                hidden = true;
            }
            if flags.contains('F') {
                dash = true;
            }
            if flags.contains('l') {
                dashl = true;
            }
            if flags.contains('r') {
                rev = true;
            }
        }
        None => {}
    }

    handle_files_folders(&mut files, &mut folders, args, dashl);

    // println!("{}   {}", files.len(), folders.len());

    sort_fd(&mut files);
    sort_fd(&mut folders);

    // println!("{:?}", folders);

    if dashl {
        long_ls(files.clone(), folders.clone(), hidden, dash, rev);
    } else {
        default_ls(files.clone(), folders.clone(), hidden, dash, rev);
    }
}

pub fn handle_files_folders(
    files: &mut Vec<String>,
    folders: &mut Vec<String>,
    args: &mut Vec<String>,
    dashl: bool
) {
    for i in args {
        let path = create_path(String::from("."), i.clone());
        
        match path.symlink_metadata() {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() {
                    if dashl {
                        files.push(i.clone());
                        if !path.exists() {
                            println!("ls: cannot read symbolic link '{}': Permission denied", i);
                        }
                    } else {
                        match path.metadata() {
                            Ok(target_metadata) => {
                                if target_metadata.is_dir() {
                                    folders.push(i.clone());
                                } else {
                                    files.push(i.clone());
                                }
                            }
                            Err(_) => {
                                files.push(i.clone());
                                println!("ls: cannot read symbolic link '{}': Permission denied", i);
                            }
                        }
                    }
                } else if metadata.is_dir() {
                    folders.push(i.clone());
                } else {
                    files.push(i.clone());
                }
            }
            Err(_) => {
                println!("ls: cannot access '{}': No such file or directory", i);
                continue;
            }
        }
    }
}

pub fn default_ls(files: Vec<String>, folders: Vec<String>, hidden: bool, dash: bool, rev : bool) {
    display_files(String::from("."), files.clone(), folders.len() > 0, dash, rev);
    display_folders(folders.clone(), files.len() != 0 || folders.len() > 1, hidden, dash, rev);
}

pub fn long_ls(files: Vec<String>, folders: Vec<String>, hidden: bool, dash: bool, rev : bool) {
    display_long_files(String::from("."), files.clone(), folders.len() > 0, dash, rev);
    display_long_folders(folders.clone(), files.len() != 0 || folders.len() > 1, hidden, dash, rev);
}

pub fn display_files(parent: String, files: Vec<String>, cc: bool, dash: bool, rev : bool) {
    let mut files = files;
    if rev {
        files.reverse();
    }

    if files.is_empty() {
        return;
    }

    for (i,file) in files.iter().enumerate() {
        let path = create_path(parent.clone(), file.clone());
        let dash_suffix = dash_f(path.clone(), dash);
        let colored_name = color(file.clone(), &path, false);
        println!("{}{}", colored_name, dash_suffix);
        
        if cc && i == files.len()-1 {
            println!();
        }
    }
  
}

pub fn display_folders(folders: Vec<String>, cc: bool, hidden: bool, dash: bool, rev : bool) {
    let mut jj = 0;
    let mut folders = folders ;
    if rev {
        folders.reverse();
    }
    for i in &folders {
        let a = create_path(String::from("."), i.clone());
        
        let target_path = if let Ok(metadata) = a.symlink_metadata() {
            if metadata.file_type().is_symlink() {
                match a.canonicalize() {
                    Ok(canonical_path) => canonical_path,
                    Err(_) => {
                        println!("ls: cannot access '{}': No such file or directory", i);
                        jj += 1;
                        continue;
                    }
                }
            } else {
                a.clone()
            }
        } else {
            a.clone()
        };
        
        let mut aa: Vec<_> = match read_dir(&target_path) {
            Ok(entries) => entries.collect(),
            Err(_) => {
                println!("ls: cannot open directory '{}': Permission denied", i);
                jj += 1;
                continue;
            }
        };

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

        display_files(target_path.to_string_lossy().to_string(), new_fold, jj != folders.clone().len() - 1, dash, rev);

        jj += 1;
    }
}


pub fn sort_fd(a: &mut Vec<String>) {
    a.sort_by(|a, b| compare(a, b));
}

fn compare(a: &str, b: &str) -> Ordering {
    let aa: String = a.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();
    let bb: String = b.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();

    match aa.cmp(&bb) {
        Ordering::Equal => {
            let aaa = a.to_lowercase();
            let bbb = b.to_lowercase();

            match aaa.cmp(&bbb) {
                Ordering::Equal => {
                    match count_lower(a).cmp(&count_lower(b)) {
                        Ordering::Equal => {
                            let a_modif = Path::new(a).metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
                            let b_modif = Path::new(b).metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);

                            b_modif.cmp(&a_modif)
                        }
                        other => other.reverse(),
                    }
                }
                other => other,
            }
        }
        other => other,
    }
}

fn count_lower(s: &str) -> usize {
    s.chars().filter(|c| c.is_lowercase()).count()
}


pub fn create_path(a: String, c : String) -> PathBuf {
    if c.starts_with("/") {
        PathBuf::from(c)
    } else {
        PathBuf::from(a).join(c)
    }
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
        return String::new();
    };

    let file_type = metadata.file_type();

    if file_type.is_symlink() {
        match path.metadata() {
            Ok(target_metadata) => {
                if target_metadata.is_dir() {
                    return String::from("/");  
                } else if target_metadata.is_file() {
                    let permissions = target_metadata.permissions();
                    let mode = permissions.mode();
                    if (mode & 0o111) != 0 {
                        return String::from("*");  
                    } else {
                        return String::new();  
                    }
                } else if target_metadata.file_type().is_fifo() {
                    return String::from("|"); 
                } else if target_metadata.file_type().is_socket() {
                    return String::from("=");  
                } else {
                    return String::new();  
                }
            }
            Err(_) => {
                return String::new();
            }
        }
    }

    if file_type.is_dir() {
        return String::from("/");
    }

    if file_type.is_file() {
        let permissions = metadata.permissions();
        let mode = permissions.mode();

        if (mode & 0o111) != 0 {
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

pub fn display_long_files(parent: String, files: Vec<String>, cc: bool, dash: bool, rev : bool) {
    let mut files = files;
    if rev {
        files.reverse();
    }
    let mut permissions_vec = Vec::new();
    let mut links_vec = Vec::new();
    let mut username_vec = Vec::new();
    let mut groupname_vec = Vec::new();
    let mut size_vec = Vec::new();
    let mut mod_time_vec = Vec::new();
    let mut major_vec = Vec::new();
    let mut dashf = Vec::new();
    let mut maj = false;

    // println!("{:?}", files.clone());

    for file in &files {
        let path = create_path(parent.clone(), file.clone());

        let Ok(metadata) = path.symlink_metadata() else {
            continue;
        };

        let perms = permissions(path.clone());

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

        let mod_time = format_time(&metadata.modified().unwrap_or(std::time::SystemTime::now()));

        let d = dash_f(path.clone(), dash);

        dashf.push(d.clone());

        permissions_vec.push(perms.clone());
        links_vec.push(links.to_string());
        username_vec.push(username.clone());
        groupname_vec.push(groupname.clone());
        size_vec.push(size.to_string());
        mod_time_vec.push(mod_time.clone());

        if metadata.file_type().is_char_device() || metadata.file_type().is_block_device() {
            maj = true;
            let rdev = metadata.rdev();
            let major_num = (rdev >> 8) & 0xfff;
            let minor_num = (rdev & 0xff) | ((rdev >> 12) & 0xfff00);
            major_vec.push(format!("{},", major_num));
            size_vec.pop();
            size_vec.push(format!("{}", minor_num));
        } else {
            major_vec.push(String::from(""));
        }
    }

    // println!("{}  {}   {}", permissions_vec.len(), major_vec.len(), size_vec.len());

    let max_perms = permissions_vec.iter().map(|s| s.len()).max().unwrap_or(0);
    let max_links = links_vec.iter().map(|s| s.len()).max().unwrap_or(0);
    let max_username = username_vec.iter().map(|s| s.len()).max().unwrap_or(0);
    let max_groupname = groupname_vec.iter().map(|s| s.len()).max().unwrap_or(0);
    let max_size = size_vec.iter().map(|s| s.len()).max().unwrap_or(0);
    let max_mod_time = mod_time_vec.iter().map(|s| s.len()).max().unwrap_or(0);
    let max_major = major_vec.iter().map(|s| s.len()).max().unwrap_or(0);

    for i in 0..files.len() {
        let major_field = if i < major_vec.len() && !major_vec[i].is_empty() {
            format!(" {:>max_major$}", major_vec[i])
        } else if maj {
            format!(" {:>max_major$}", "")
        } else {
            String::new()
        };


        print!(
            "{:<max_perms$} {:>max_links$} {:<max_username$} {:<max_groupname$}{} {:>max_size$} {:<max_mod_time$} {}",
            permissions_vec[i],
            links_vec[i],
            username_vec[i],
            groupname_vec[i],
            major_field,
            size_vec[i],
            mod_time_vec[i],
            if quotes(files[i].clone()).1 {
                color(quotes(files[i].clone()).0, &create_path(parent.clone(), files[i].clone()), false)
            } else {
                color(format!("{}", files[i].clone()), &create_path(parent.clone(), files[i].clone()), false)
            },
        );
        let targett = read_link_target(&create_path(parent.clone(), files[i].clone()));
        if let Some(target) = targett {
            let target_dash = dash_f(create_path(parent.clone(), target.clone()), dash);
            println!(
                " -> {}{}",
                color(target.clone(), &create_path(parent.clone(), target.clone()), true),
                target_dash
            );
        } else {
            println!("{}", dashf[i]);
        }
        if cc && i == files.len() - 1 {
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

    let acl = if has_acl(&path) { String::from("+") } else { String::new() };

    let owner_r = if (mode & 0o400) != 0 { 'r' } else { '-' };
    let owner_w = if (mode & 0o200) != 0 { 'w' } else { '-' };
    let owner_x = match ((mode & 0o100) != 0, (mode & 0o4000) != 0) {
        (true, true) => 's',
        (false, true) => 'S',
        (true, false) => 'x',
        (false, false) => '-',
    };

    let group_r = if (mode & 0o040) != 0 { 'r' } else { '-' };
    let group_w = if (mode & 0o020) != 0 { 'w' } else { '-' };
    let group_x = match ((mode & 0o010) != 0, (mode & 0o2000) != 0) {
        (true, true) => 's',
        (false, true) => 'S',
        (true, false) => 'x',
        (false, false) => '-',
    };

    let other_r = if (mode & 0o004) != 0 { 'r' } else { '-' };
    let other_w = if (mode & 0o002) != 0 { 'w' } else { '-' };
    let other_x = match ((mode & 0o001) != 0, (mode & 0o1000) != 0) {
        (true, true) => 't',
        (false, true) => 'T',
        (true, false) => 'x',
        (false, false) => '-',
    };

    format!(
        "{}{}{}{}{}{}{}{}{}{}{acl}",
        type_char,
        owner_r,
        owner_w,
        owner_x,
        group_r,
        group_w,
        group_x,
        other_r,
        other_w,
        other_x
    )
}

fn has_acl(path: &Path) -> bool {
    let path_cstr = match CString::new(path.as_os_str().as_bytes()) {
        Ok(cstr) => cstr,
        Err(_) => {
            return false;
        }
    };

    unsafe {
        let result = libc::getxattr(
            path_cstr.as_ptr(),
            b"system.posix_acl_access\0".as_ptr() as *const i8,
            std::ptr::null_mut(),
            0
        );
        result > 0
    }
}

pub fn display_long_folders(folders: Vec<String>, cc: bool, hidden: bool, dash: bool, rev : bool) {
    let mut jj = 0;
    let mut folders = folders ;
    if rev {
        folders.reverse();
    }
    for i in &folders {
        let a = create_path(String::from("."), i.clone());
        
        let target_path = if let Ok(metadata) = a.symlink_metadata() {
            if metadata.file_type().is_symlink() {
                match a.canonicalize() {
                    Ok(canonical_path) => canonical_path,
                    Err(_) => {
                        println!("ls: cannot access '{}': No such file or directory", i);
                        jj += 1;
                        continue;
                    }
                }
            } else {
                a.clone()
            }
        } else {
            a.clone()
        };

        let Ok(aaa) = read_dir(&target_path) else {
            println!("ls: cannot open directory '{}': Permission denied", i);
            jj += 1;
            continue;
        };

        let mut aa: Vec<_> = aaa.collect();

        let mut total = 0;

        if !hidden {
            aa.retain(|x| !x.as_ref().unwrap().file_name().to_string_lossy().starts_with("."));
            total += total_blocks(&aa);

        } else {
            total += total_blocks(&aa);
            // println!("+++ total {}", total);
            if let Ok(current_metadata) = target_path.metadata() {
                total += current_metadata.blocks() / 2;
                // println!("sss");
            }
            // println!("--- total {}", total);

            // println!("+-+- {:?}", target_path);

            let parent_path = target_path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("..")).to_path_buf();

            // println!("*** parent_path: {:?}", parent_path);
            
            // println!("*** parent: {:?}", parent_path);
            if let Ok(parent_metadata) = parent_path.metadata() {
                total += parent_metadata.blocks() / 2;
                // println!("ttt");
            }
            // println!("*** total {}", total);

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

        display_long_files(target_path.to_string_lossy().to_string(), new_fold, jj != folders.clone().len() - 1, dash, rev);

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

    total / 2
}

pub fn format_time(time: &std::time::SystemTime) -> String {
    let now: DateTime<Utc> = (*time).into();
    let mor_now = now.with_timezone(&Casablanca);
    let six_months_ago = Utc::now().with_timezone(&Casablanca) - chrono::Duration::days(182);

    if mor_now < six_months_ago {
        mor_now.format("%b %e  %Y").to_string()
    } else {
        mor_now.format("%b %e %H:%M").to_string()
    }
}

pub fn quotes(filename: String) -> (String, bool) {
    // let special: &[char] = &[
    //     ' ', '\t', '\n', '\'', '"', '\\', '|', '&', ';', '(', ')', 
    //     '<', '>', '*', '?', '[', ']', '{', '}', '$', '`', '!', 
    //     '#', '~', '=', '%', '^'
    // ];

    (filename,false)
    
    // if !filename.is_empty() && 
    //    !filename.starts_with('-') && 
    //    filename.chars().all(|c| !c.is_control() && !special.contains(&c)) {
    //     return (filename, false);
    // }

    // (format!("'{}'", filename), true)
}

pub fn color(a: String, path: &PathBuf , target : bool) -> String {
    let Ok(link) = path.symlink_metadata() else {
        return format!("\x1b[1;31;40m{}\x1b[0m", a);
    };

    
    let metadata = if link.file_type().is_symlink() {
        match path.metadata() {
            Ok(m) => if target { m } else { return format!("\x1b[1;36m{}\x1b[0m", a) },
            Err(_) =>  return format!("\x1b[1;31;40m{}\x1b[0m", a),
        }
    } else {
        link
    };

    // if metadata.file_type().is_symlink() && !target {
    //     if let Ok(_) = path.read_link() {
    //         ;
    //     }
    //         return format!("\x1b[31;40m{}\x1b[0m", a);
    // };

    let file_type = metadata.file_type();

    if file_type.is_dir() {
        return format!("\x1b[1;34m{}\x1b[0m", a);
    }

    if file_type.is_file() {
        let permissions = metadata.permissions();
        let mode = permissions.mode();

        if (mode & 0o111) != 0 {
            return format!("\x1b[1;32m{}\x1b[0m", a);
        }
        return a;
    }

    if file_type.is_fifo() {
        return format!("\x1b[0;40;33m{}\x1b[0m", a);
    }

    if file_type.is_socket() {
        return format!("\x1b[1;35m{}\x1b[0m", a);
    }

    if file_type.is_char_device() || file_type.is_block_device() {
        return format!("\x1b[1;40;33m{}\x1b[0m", a); 
    }

    a 
}
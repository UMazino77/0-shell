use crate::zero::Commands;
use crate::zero::*;

pub fn exec_cp(
    cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    if args.len() < 2 {
        eprintln!("Usage: cp SOURCE DEST");
        return;
    }

    detect_flags(cmd.clone(), args, mp);

    // println!("{:?}", mp) ;

    if !valid_flags(cmd.clone(), mp) {
        eprintln!("cp: invalid option");
        return;
    }

    // println!("{:?} --- {:?}      +++++++++++++", args, mp);

    let src = &args[0];
    let dest = &args[1];

    let a = "./".to_owned() + src;
    let b = "./".to_owned() + dest;

    let src_path = std::path::Path::new(&a);
    let dest_path = std::path::Path::new(&b);

    
    if !src_path.exists() {
        eprintln!("cp: cannot stat '{}': No such file or directory", src);
        return;
    }
    
    // println!("{:?} --- {:?}", src_path.display(), dest_path.display());
    // println!("{:?} --- {:?}", src_path.is_dir(), src_path.is_file());

    if src_path.is_dir() {
        if mp.contains_key(&Commands::Cp) && mp.get(&Commands::Cp) == Some(&"r".to_string()) {
            if let Err(e) = std::fs::create_dir_all(dest_path) {
                eprintln!("cp: error creating directory '{}': {}", dest, e);
                return;
            }
            for i in std::fs::read_dir(src_path).unwrap() {
                let i = i.unwrap();
                let file_name = i.file_name();
                let new_dest = dest_path.join(file_name);
                exec_cp(
                    cmd.clone(),
                    &mut vec![i.path().to_str().unwrap().to_string(), new_dest.to_str().unwrap().to_string()],
                    mp
                );
            }
        } else {
            eprintln!("cp: -r not specified; omitting directory '{}'", src);
            return ;
        }
    } else {
        if let Err(e) = std::fs::copy(src_path, dest_path) {
            eprintln!("cp: error copying file '{}': {}", src, e);
        }
    }

}

use crate::zero::Commands;
use crate::zero::*;

pub fn exec_cat(
    _cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
) {
    if args.len() < 1 {
        println!("cat: missing operand");
        return;
    }
    detect_flags(Commands::Cat, args, mp);
    if !valid_flags(Commands::Cat, mp) {
        println!("cat: invalid option");
        return;
    }
    let file_name = &args[0];
    let path = std::path::Path::new(file_name);
    if !path.exists() {
        eprintln!("cat: {}: No such file or directory", file_name);
        return;
    }
    if path.is_dir() {
        eprintln!("cat: {}: Is a directory", file_name);
        return;
    }
    if mp.contains_key(&Commands::Cat) && mp.get(&Commands::Cat) == Some(&"n".to_string()) {
        
        if let Ok(contents) = std::fs::read_to_string(file_name) {
            let mut j = 0 ;
            for line in contents.lines() {
                if line.trim().is_empty() || (line == "#V2".to_owned() && j == 0 && args[0] == "history.txt") {
                    continue;
                }
                println!("{:>6}  {}", j+1, line);
                j += 1;
            }
        } else {
            eprintln!("cat: {}: Error reading file", file_name);
        }
        return;
    }
    match std::fs::read_to_string(file_name) {
        Ok(contents) => print!("{}", contents),
        Err(e) => eprintln!("cat: {}: {}", file_name, e),
    }
}
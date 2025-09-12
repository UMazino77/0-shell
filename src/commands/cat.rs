use crate::zero::Commands;
use crate::zero::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

pub fn exec_cat(
    init : &mut i32,
    _cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut HashMap<Commands, String>
)-> Result<(), Box<dyn std::error::Error>> {

    if args.len() > 0 && args[0] == "--" {
        args.remove(0);
    }

    if args.len() < 1 || args[0] == "-" {
        return empty_cat(false, init);
    }

    detect_flags(Commands::Cat, args, mp);
    if !valid_flags(Commands::Cat, mp) {
        return Ok(());
    }

    if args.len() < 1 {
        return empty_cat(true,init);
    }

    let file_name = &args[0];
    let path = std::path::Path::new(file_name);
    if !path.exists() {
        eprintln!("cat: '{}': No such file or directory", file_name);
        return Ok(());
    }
    if path.is_dir() {
        eprintln!("cat: {}: Is a directory", file_name);
        return Ok(());
    }
    if mp.contains_key(&Commands::Cat) && mp.get(&Commands::Cat) == Some(&"n".to_string()) {
        let mut j = 0 ;
        if let Ok(contents) = std::fs::read_to_string(file_name) {
            for line in contents.lines() {
                if line.trim().is_empty() || (line == "#V2".to_owned() && j == 0 && args[0] == "history.txt") {
                    continue;
                }
                println!("{:>6}  {}", init, line);
                *init += 1;
                j += 1;
            }
        } else {
            eprintln!("cat: {}: Error reading file", file_name);
        }
        return Ok(());
    }
    match std::fs::read_to_string(file_name) {
        Ok(contents) => eprintln!("{}", contents),
        Err(e) => eprintln!("cat: {}: {}", file_name, e),
    }
    Ok(())
}

pub fn empty_cat(dash :bool , init : &mut i32) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}",  dash);
    let mut rl = Editor::<(), _>::new()?;
    loop {
        match rl.readline("") {
            Ok(line) => {
                if dash {
                    println!("{:>6}  {}", init, line);
                    *init += 1;
                    _ = exec_cat(init, Commands::Cat, &mut vec!["-n".to_string()], &mut HashMap::new());
                } else {
                    eprintln!("{line}");
                    _ = exec_cat(init, Commands::Cat, &mut vec!["-".to_string()], &mut HashMap::new());
                }
                return Ok(());
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                return Ok(());
            }
            Err(_) => {
                return Ok(());
            }
        };
    }
}

        
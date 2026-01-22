// use crate::commands::echo;
use rustyline::Editor;
use crate::zero::Commands;
use crate::zero::*;
use std::collections::HashMap;
use std::error::Error;
use rustyline::error::ReadlineError;

pub fn exec_cat(
    init: &mut i32,
    _cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut HashMap<Commands, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // println!("salam");
    detect_flags(Commands::Cat, args, mp);
    if !valid_flags(Commands::Cat, mp) {
        return Ok(());
    }

    if args.len() < 1 {
        return empty_cat(true, init);
    }



    for file in args {
        let aaa:Result<(), Box<dyn Error>> ;
        // println!("file :  {}", file);
        // println!("{} ===> {}", mp.contains_key(&Commands::Cat), file);
        if file == "-" || file == "--" {
            aaa = empty_cat(mp.contains_key(&Commands::Cat), init);
            if aaa.is_err() {
                return Ok(()) ;    
            } else if !aaa.is_err() {
                continue;
            }
        }
        let path = std::path::Path::new(&file);
        if !path.exists() {
            eprintln!("cat: '{}': No such file or directory", file.clone());
            return Ok(());
        }
        if path.is_dir() {
            eprintln!("cat: {}: Is a directory", file.clone());
            return Ok(());
        }
        if mp.contains_key(&Commands::Cat) && mp.get(&Commands::Cat) == Some(&"n".to_string()) {
            if let Ok(contents) = std::fs::read_to_string(file.clone()) {
                for line in contents.lines() {
                    println!("{:>6}  {}", init, line);
                    *init += 1;
                }
            } else {
                eprintln!("cat: {}: Error reading file", file.clone());
            }
        } else {
            match std::fs::read_to_string(file.clone()) {
                Ok(contents) => eprintln!("{}", contents),
                Err(e) => eprintln!("cat: {}: {}", file.clone(), e),
            }
        }
    }
    Ok(())
}



pub fn empty_cat(dash: bool, init: &mut i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<(), _>::new()?;

    loop {
        match rl.readline("") {
            Ok(line) => {
                if dash {
                    println!("{:>6}  {}", init, line);
                    *init += 1;
                } else {
                    println!("{}", line);
                }
            },
            Err(ReadlineError::Eof) => {
                return Ok(());
            },
            _ => {
                return Err(("salam").into());
            }
        }
    }
}

use crate::zero::Commands;
use crate::zero::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn exec_cat(
    _cmd: Commands,
    args: &mut Vec<String>,
    mp: &mut std::collections::HashMap<Commands, String>
)-> Result<(), Box<dyn std::error::Error>> {

    if args.len() > 0 && args[0] == "--" {
        args.remove(0);
    }

    if args.len() < 1 || args[0] == "-" {

        let mut rl = Editor::<(), _>::new()?;
        match rl.readline("") {
            Ok(line) => {
                eprintln!("{line}");
                let _ = exec_cat(_cmd, args, mp);
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                return Ok(());
            }
           
            Err(_) => {
                return Ok(());
            }
        };
        return Ok(())
    }

    detect_flags(Commands::Cat, args, mp);
    if !valid_flags(Commands::Cat, mp) {
        println!("cat: invalid option");
        return Ok(());
    }
    let file_name = &args[0];
    let path = std::path::Path::new(file_name);
    if !path.exists() {
        eprintln!("cat: {}: No such file or directory", file_name);
        return Ok(());
    }
    if path.is_dir() {
        eprintln!("cat: {}: Is a directory", file_name);
        return Ok(());
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
        return Ok(());
    }
    match std::fs::read_to_string(file_name) {
        Ok(contents) => eprintln!("{}", contents),
        Err(e) => eprintln!("cat: {}: {}", file_name, e),
    }
    Ok(())
}
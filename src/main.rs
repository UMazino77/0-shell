use shell::zero::*;
use std::collections::HashMap;
use rustyline::{Editor, error::ReadlineError};
use shell::commands::echo::exec_echo;

fn main() -> rustyline::Result<()> {
    let mut rl = Editor::<(),_>::new()?;
    let _ = rl.load_history("0shell_history.txt");
    
    let mut mp = HashMap::new();
    loop {
        let mut path = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        let user = whoami::username();
        path  = path.replace(&("/home/".to_owned() + &user), "~");
        
        let ar = match rl.readline(&format!("{} $ ", path)) {
            Ok(mut line) => {
                let aaa  = line.trim().split_whitespace().collect::<Vec<&str>>();
                if aaa[0] == "echo" {
                    exec_echo(Commands::Echo, &mut aaa[1..].iter().map(|x| x.to_string()).collect(), &mut mp, &mut line);
                }
               let _ = rl.add_history_entry(line.as_str());
                let _ = rl.append_history("history.txt");
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(_) => continue,
        };
        
        let args: Vec<&str> = ar.trim().split(|x:char| x == ';').collect();
        if args.len() < 1 {
            println!("Usage: <command> [args...]");
            // std::process::exit(1);
        }
        let mut b: Vec<Vec<String>> = Vec::new();
        for i in args.iter() {
            let a = i.split_whitespace().map(|x| x.to_string()).collect();
            b.push(a);
        }
        for j in b.iter_mut() {
            if j.len() < 1 {
                println!("$");
                continue;
            }
            match Commands::from_str(&j[0]) {
                Some(cmd) => {
                    if cmd != Commands::Echo {
                        execute(cmd, &mut j[1..].to_owned(), &mut mp);
                    }
                }
                None => {
                    println!("Unknown command: {}", j[0]);
                }
            }
        }
        // println!("{:?}", b);
    }
    
    Ok(())
}
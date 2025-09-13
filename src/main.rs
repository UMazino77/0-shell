use rustyline::{Editor, error::ReadlineError};
use shell::commands::echo::exec_echo;
use shell::lexer::token::{has_unclosed_quotes, tokenize_input};
use shell::zero::*;
use std::collections::HashMap;

fn main() -> rustyline::Result<()> {
    let mut rl = Editor::<(), _>::new()?;
    let user = whoami::username();
    let his_path = format!("/home/{}/.zero-history.txt", user);
    let _ = rl.load_history(&his_path);
    let mut mp = HashMap::new();
    // println!("hello'$'\n''world");

    loop {
        let a = mp.get(&Commands::Pwd);
        let p = std::env::current_dir();
        // let mut path = if !mp.contains_key(&Commands::Pwd)
        let mut path = if let Ok(pp) = p && !mp.contains_key(&Commands::Pwd) {
            pp.display().to_string()
        } else {
            a.unwrap_or(&"Unknown error".to_string()).to_string()
        };
        path = path.replace(&("/home/".to_owned() + &user), "~");

        let mut line = match rl.readline(&format!("{}:{} $ ", col_user(), col_path(path))) {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let _ = rl.append_history(&his_path);
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(_) => {
                continue;
            }
        };

        // Check for unclosed quotes using lexer
        while has_unclosed_quotes(&line) {
            match rl.readline("dquote> ") {
                Ok(additional_input) => {
                    line.push('\n');
                    line.push_str(&additional_input);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    line = String::new();
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("^D");
                    line = String::new();
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        }

        let mut b = tokenize_input(&line, &user);

        remove_quotes(&mut b);

        for j in b.iter_mut() {
            match Commands::from_str(&j[0]) {
                Some(Commands::Echo) => {
                    exec_echo(Commands::Echo, &mut j[1..].to_owned(), &mut mp);
                }
                Some(cmd) => {
                    execute(cmd, &mut j[1..].to_owned(), &mut mp);
                }
                None => {
                    println!("Command '{}' not found", j[0]);
                }
            }
        }
    }

    Ok(())
}

// Keep utility functions
pub fn col_user() -> String {
    let user = whoami::username();
    format!("\x1b[1;32m{}\x1b[0m", user)
}

pub fn col_path(a: String) -> String {
    format!(
        "\x1b[1;33m[<\x1b[0m\x1b[1;1m{}\x1b[0m\x1b[1;33m>]\x1b[0m",
        a
    )
}

pub fn remove_quotes(args: &mut Vec<Vec<String>>) {
    // let l = args.len();
    for a in args.iter_mut() {
        for aa in a.iter_mut() {
            aa.retain(|c| c != '\'' && c != '"');
        }
    }
    args.retain(|a| !a.is_empty());

    // if args.len() != l {
    //     println!("syntax error near unexpected token `;'");
    // }
}
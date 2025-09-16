use rustyline::{Editor, error::ReadlineError};
use shell::lexer::token::{has_unclosed_quotes, tokenize_input};
use shell::zero::*;
use std::collections::HashMap;

fn main() -> rustyline::Result<()> {
    let mut editor = Editor::<(), _>::new()?;
    let user = whoami::username();
    let his_path = format!("/home/{}/.zero-history.txt", user);
    let _ = editor.load_history(&his_path);
    let mut cmd_map = HashMap::new();

    loop {
        let last_place = cmd_map.get(&Commands::Pwd);
        let current_place = std::env::current_dir();
        let mut path = if let Ok(place) = current_place && !cmd_map.contains_key(&Commands::Pwd) {
            place.display().to_string()
        } else {
            last_place.unwrap_or(&"Unknown error".to_string()).to_string()
        };
        path = path.replace(&("/home/".to_owned() + &user), "~");

        let mut line = match editor.readline(&format!("{}:{} $ ", col_user(), col_path(path))) {
            Ok(line) => {
                let _ = editor.add_history_entry(line.as_str());
                let _ = editor.append_history(&his_path);
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(_) => {
                break;
            }
        };

        while has_unclosed_quotes(&line) {
            match editor.readline("dquote> ") {
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

        let mut input = tokenize_input(&line, &user);

        for line in input.iter_mut() {
            match Commands::from_str(&line[0]) {
                Some(cmd) => {
                    execute(cmd, &mut line[1..].to_owned(), &mut cmd_map);
                }
                None => {
                    println!("Command '{}' not found", line[0]);
                }
            }
        }
    }
    Ok(())
}

pub fn col_user() -> String {
    let user = whoami::username();
    format!("\x1b[1;32m{}\x1b[0m", user)
}

pub fn col_path(path: String) -> String {
    format!(
        "\x1b[1;33m[<\x1b[0m\x1b[1;1m{}\x1b[0m\x1b[1;33m>]\x1b[0m",
        path
    )
}

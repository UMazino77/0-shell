use shell::zero::*;
use std::collections::HashMap;
use rustyline::{ Editor, error::ReadlineError };
use shell::commands::echo::exec_echo;

fn main() -> rustyline::Result<()> {
    let mut rl = Editor::<(), _>::new()?;
    let user = whoami::username();
    let his_path = format!("/home/{}/.zero-history.txt", user);
    let _ = rl.load_history(&his_path);
    let mut mp = HashMap::new();
    
    loop {
        let mut path = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        path = path.replace(&("/home/".to_owned() + &user), "~");

        let mut line = match rl.readline(&format!("{} $ ", path)) {
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
                // clear_terminal();
                break;
            }
            Err(_) => {
                continue;
            }
        };

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

        let args: Vec<&str> = line
            .trim()
            .split(|x: char| x == ';')
            .collect();
            
        if args.is_empty() {
            continue;
        }

        let mut b: Vec<Vec<String>> = Vec::new();
        for i in args.iter() {
            if i.trim().is_empty() {
                continue;
            }
            
            let a = handle_quotes(i.trim(), &user);
            if !a.is_empty() {
                b.push(a);
            }
        }

        for j in b.iter_mut() {
            if j.is_empty() {
                continue;
            }

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

fn has_unclosed_quotes(input: &str) -> bool {
    let mut aaa = 0;
    let mut bbb = 0;
    let mut escaped = false;
    
    for c in input.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        
        match c {
            '\\' => escaped = true,
            '"' => aaa += 1,
            '\'' => bbb += 1,
            _ => {}
        }
    }
    
    aaa % 2 != 0 || bbb % 2 != 0
}

fn handle_quotes(input: &str, user: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut ress = String::new();
    let mut aaa = 0;
    let mut bbb = 0;
    let mut escaped = false;
    
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let c = chars[i];
        
        if escaped {
            match c {
                '"' | '\'' | '\\' | 'n' | 't' | 'r' => {
                    match c {
                        'n' => ress.push('\n'),
                        't' => ress.push('\t'),
                        'r' => ress.push('\r'),
                        _ => ress.push(c),
                    }
                }
                _ => {
                    ress.push('\\');
                    ress.push(c);
                }
            }
            escaped = false;
        } else {
            match c {
                '\\' => {
                    escaped = true;
                }
                '"' if bbb % 2 == 0 => {
                    aaa += 1;
                }
                '\'' if aaa % 2 == 0 => {
                    bbb += 1;
                }
                ' ' | '\t' if aaa % 2 == 0 && bbb % 2 == 0 => {
                    if !ress.is_empty() {
                        if ress.starts_with('~') {
                            ress = ress.replace("~", &format!("/home/{}", user));
                        }
                        res.push(ress.replace("\n", "'$'\\n"));
                        ress = String::new();
                    }
                    while i + 1 < chars.len() && (chars[i + 1] == ' ' || chars[i + 1] == '\t') {
                        i += 1;
                    }
                }
                _ => {
                    ress.push(c);
                }
            }
        }
        i += 1;
    }
    
    if !ress.is_empty() {
        if ress.starts_with('~') {
            ress = ress.replace("~", &format!("/home/{}", user));
        }
        res.push(ress.replace("\n", "'$'\\n"));
    }
    
    res
}
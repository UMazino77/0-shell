use crate::zero::Commands;
use rustyline::*;
use rustyline::error::ReadlineError;

pub fn exec_echo(
    _cmd: Commands,
    line: &mut String,
    _mp: &mut std::collections::HashMap<Commands, String>
) {
    if line.replace("echo", "").trim().is_empty() {
        println!();
        return;
    }
    let mut output = line.clone().replace("echo", "").trim().to_string();
    let mut cc = Editor::<(), _>::new().expect("Failed to create editor");

    if (output.len() - output.replace("\"", "").len()) % 2 == 1 {
        match cc.readline("dquote> ") {
            Ok(additional_input) => {
                line.push('\n');
                line.push_str(&additional_input);
                return exec_echo(_cmd, line, _mp);
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                return;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                return;
            }
            Err(_) => {
                return;
            }
        }
    }

    handle_quotes(&mut output);
    println!("{}", output);
}

pub fn handle_quotes(input: &mut String) {
    let mut res = String::new();
    let mut aaa = 0;
    let mut bbb = 0;
    let mut i = 0;
    let chars = input.chars().collect::<Vec<_>>();

    while i < chars.len() {
        let c = chars[i];
        match c {
            '"' => {
                aaa += 1;
            }
            '\'' => {
                bbb += 1;
            }
            '\\' if aaa % 2 == 1 || bbb % 2 == 1 => {
                if i + 1 < chars.len() {
                    let next_char = chars[i + 1];
                    if next_char == '"' || next_char == '\\' || next_char == '\'' {
                        res.push(next_char);
                        i += 1;
                    } else {
                        res.push(c);
                    }
                } else {
                    res.push(c);
                }
            }
            _ => {
                res.push(c);
            }
        }
        i += 1;
    }

    *input = res;
}

use crate::zero::Commands;
use rustyline::*;
use rustyline::error::ReadlineError;

pub fn exec_echo(
    _cmd: Commands,
    line: &mut String,
    _mp: &mut std::collections::HashMap<Commands, String>,
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
            Err(_) => return,
        }
    }

    output = output.replace("\\n", "\n").replace("\\t", "\t").replace("\"", "").replace("\\\"", "\"");
    println!("{}", output);
}
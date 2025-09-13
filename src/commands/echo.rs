use crate::zero::Commands;
// use crate::zero::*;


pub fn exec_echo(
    _cmd: Commands,
    args: &mut Vec<String>,
    _mp: &mut std::collections::HashMap<Commands, String>
) {
  

    // detect_flags(Commands::Echo, args, _mp);
    // if !valid_flags(Commands::Echo, _mp) {
    //     return;
    // }

    let processed_args: Vec<String> = args.iter()
        .map(|arg| process_escape_sequences(arg))
        .collect();

    let output = processed_args.join(" ");
    println!("{}", output);
}

fn process_escape_sequences(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    let mut escaped = false;
    
    while let Some(c) = chars.next() {
        if escaped {
            match c {
                'n' => result.push('\n'),
                't' => result.push('\t'),
                'r' => result.push('\r'),
                // 'f' => result.push('\f'),
                '\\' => result.push('\\'),
                _ => {
                    result.push('\\');
                    result.push(c);
                }
            }
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else {
            result.push(c);
        }
    }
    
    if escaped {
        result.push('\\');
    }
    
    result
}
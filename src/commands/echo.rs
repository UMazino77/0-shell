use crate::zero::Commands;
use crate::zero::*;

// pub fn exec_echo(
//     _cmd: Commands,
//     args: &mut Vec<String>,
//     _mp: &mut std::collections::HashMap<Commands, String>
// ) {
//     if args.is_empty() {
//         println!();
//         return;
//     }

//     detect_flags(Commands::Echo, args, _mp);
//     if !valid_flags(Commands::Echo, _mp) {
//         return;
//     }

//     let output = args.join(" ");
//     println!("{}", output);
// }
// use crate::zero::Commands;
// use crate::zero::*;

pub fn exec_echo(
    _cmd: Commands,
    args: &mut Vec<String>,
    _mp: &mut std::collections::HashMap<Commands, String>
) {
  

    detect_flags(Commands::Echo, args, _mp);
    if !valid_flags(Commands::Echo, _mp) {
        return;
    }

    // Process escape sequences in each argument
    let processed_args: Vec<String> = args.iter()
        .map(|arg| process_escape_sequences(arg))
        .collect();

    let output = processed_args.join(" ");
    println!("{}", output);
}

// Add this function to handle escape sequences
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
                // '\'' => result.push('\''),
                // '0' => result.push('\0'),
                // ' ' => result.push(' '),    // Escaped space
                // '$' => result.push('$'),    // Escaped dollar
                // '`' => result.push('`'),    // Escaped backtick
                _ => {
                    // Unknown escape sequence, treat literally
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
    
    // Handle trailing backslash
    if escaped {
        result.push('\\');
    }
    
    result
}
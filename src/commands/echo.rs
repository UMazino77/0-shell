use crate::zero::Commands;
use rustyline::* ;

pub fn exec_echo(
    _cmd: Commands,
    args: &mut Vec<String>,
    _mp: &mut std::collections::HashMap<Commands, String>,
    line: &mut String
) {
    if args.len() == 0 {
        println!();
        return;
    }
    let mut output = args.join(" ") ;
    if (output.len() - output.replace("\"", "").len())%2 == 1 {
        let additional_input = Editor::<(),_>::new().expect("Failed to create editor").readline("").unwrap_or_default();
        eprint!(">");
        
        args.push(additional_input+"\n") ;
        return exec_echo(_cmd, args, _mp, line);
    }
    output = args.join(" ") ;
    output = output.replace("\\n", "\n").replace("\\t", "\t").replace("\"", "") ;
    *line = format!("echo {}", output);
    print!("{}", output);
}
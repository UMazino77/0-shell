use crate::zero::Commands;
use crate::zero::*;

pub fn exec_echo(
    _cmd: Commands,
    args: &mut Vec<String>,
    _mp: &mut std::collections::HashMap<Commands, String>
) {
    if args.is_empty() {
        println!();
        return;
    }

    detect_flags(Commands::Echo, args, _mp);
    if !valid_flags(Commands::Echo, _mp) {
        return;
    }

    let output = args.join(" ");
    println!("{}", output);
}
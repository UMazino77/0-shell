use crate::zero::Commands;

pub fn exec_echo(
    _cmd: Commands,
    args: &[String],
    _mp: &mut std::collections::HashMap<Commands, String>
) {
    if args.is_empty() {
        println!();
        return;
    }
    
    let output = args.join(" ");
    println!("{}", output);
}
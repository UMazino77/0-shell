use crate::zero::*;
use crate::zero::Commands;

pub fn exec_clear(
    _cmd: Commands,
    args: &mut Vec<String>,
    _mp: &mut std::collections::HashMap<Commands, String>
) {
    detect_flags(Commands::Clear, args, _mp);
    if !valid_flags(Commands::Clear, _mp) {
        return;
    }
    clearscreen::clear().expect("Failed to clear terminal");
}

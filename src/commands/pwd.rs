use std::env::current_dir;
use crate::zero::*;

pub fn exec_pwd(
    _cmd: Commands,
    _args: &mut Vec<String>,
) {
    let current_dir = current_dir().unwrap();
    println!("{}", current_dir.display());
}

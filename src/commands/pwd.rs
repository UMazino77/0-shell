pub fn exec_pwd(
    _cmd: crate::zero::Commands,
    args: &mut [String],
) {
    if args.len() > 0 {
        println!("pwd: too many arguments");
        return;
    }
    let current_dir = std::env::current_dir().unwrap();
    println!("{}", current_dir.display());
}

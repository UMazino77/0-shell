pub fn exec_exit(
    args: &mut Vec<String>,
) {
    if args.len() > 1 {
        println!("exit: too many arguments");
        return;
    }
    if args.len() == 1 {
        match args[0].parse::<i32>() {
            Ok(b) => std::process::exit(b),
            Err(_) => {
                println!("exit: {}: numeric argument required", args[0]);
                std::process::exit(255);
            }
        }
    }
    std::process::exit(0);
}

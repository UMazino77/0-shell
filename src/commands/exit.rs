// use crate::zero::clear_terminal;

pub fn exec_exit(
    args: &mut Vec<String>,
) {
    println!("exit");
    if args.len() >= 1 {
        match args[0].parse::<i32>() {
            Ok(b) => std::process::exit(b),
            Err(_) => {
                println!("0-shell: exit: {}: numeric argument required", args[0]);
                std::process::exit(255);
            }
        }
    }
    // clear_terminal();
    std::process::exit(0);
}

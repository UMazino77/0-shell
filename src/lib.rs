pub mod commands;

pub mod zero {
    use crate::commands::cd::exec_cd;
    use crate::commands::rm::exec_rm;
    use crate::commands::pwd::exec_pwd;
    use crate::commands::mkdir::exec_mkdir;
    use crate::commands::exit::exec_exit;
    use crate::commands::cp::exec_cp;
    use crate::commands::mv::exec_mv;
    use crate::commands::cat::exec_cat;
    use crate::commands::history::exec_history;
    use crate::commands::clear::exec_clear;


    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Commands {
        Ls,
        Cd,
        Pwd,
        Mkdir,
        Rm,
        Cp,
        Mv,
        Echo,
        Cat,
        Exit,
        History,
        Clear
    }

    impl Commands {
        pub fn from_str(cmd: &str) -> Option<Commands> {
            match cmd {
                "ls" => Some(Commands::Ls),
                "cd" => Some(Commands::Cd),
                "pwd" => Some(Commands::Pwd),
                "mkdir" => Some(Commands::Mkdir),
                "rm" => Some(Commands::Rm),
                "cp" => Some(Commands::Cp),
                "mv" => Some(Commands::Mv),
                "echo" => Some(Commands::Echo),
                "cat" => Some(Commands::Cat),
                "exit" => Some(Commands::Exit),
                "history" => Some(Commands::History),
                "clear" => Some(Commands::Clear),


                _ => None,
            }
        }
    }

    pub fn execute(
        cmd: Commands,
        args: &mut Vec<String>,
        mp: &mut std::collections::HashMap<Commands, String>
    ) {
        match cmd {
            Commands::Rm => {
                if let Err(e) = exec_rm(cmd, args, mp) {
                    println!("Error executing rm: {}", e);
                }
            }
            Commands::Cd => exec_cd(cmd, args, mp),
            Commands::Mv => exec_mv(cmd, args, mp),
            Commands::Pwd => exec_pwd(cmd, args),
            Commands::Mkdir => exec_mkdir(cmd, args, mp),
            Commands::Cp => exec_cp(cmd, args, mp),
            Commands::Exit => exec_exit(args),
            Commands::Cat => exec_cat(cmd, args, mp),
            Commands::Clear => exec_clear(),
            Commands::History => exec_history(cmd, args, mp),
            _ => println!("Command {:?} not implemented yet", cmd),
        }
    }

    pub fn detect_flags(
        cmd: Commands,
        args: &mut Vec<String>,
        mp: &mut std::collections::HashMap<Commands, String>
    ) {
        for arg in args.clone() {
            if arg.starts_with('-') {
                // println!("{} --- ++++", arg);
                for ch in arg[1..].chars() {
                    let amp = mp.entry(cmd.clone()).or_insert(ch.to_string());
                    // println!("{} --- ++++", amp);
                    if !amp.contains(ch) {
                        amp.push(ch);
                    }
                }
                args.retain(|arg| !arg.starts_with('-'));
            }
        }
    }

    pub fn valid_flags(
        cmd: Commands,
        mp: &mut std::collections::HashMap<Commands, String>
    ) -> bool {
        // println!("{:?} --- {:?}", cmd, mp);
        match cmd {
            Commands::Rm => {
                return check(cmd.clone(), mp, "r".to_string());
            }
            Commands::Mkdir => {
                return check(cmd.clone(), mp, "p".to_string());
            }
            Commands::Cp => {
                return check(cmd.clone(), mp, "r".to_string());
            }
            Commands::Cat => {
                return check(cmd.clone(), mp, "n".to_string());
            }
            _ => {}
        }
        true
    }

    pub fn checker(
        cmd: Commands,
        mp: &mut std::collections::HashMap<Commands, String>,
        flag: char
    ) -> bool {
        mp.remove(&cmd);
        println!("{:?}: invalid option -- '{}'", cmd, flag);
        false
    }

    pub fn check(
        cmd: Commands,
        mp: &mut std::collections::HashMap<Commands, String>,
        flags: String
    ) -> bool {
        if let Some(f) = mp.get(&cmd) {
            for ch in f.chars() {
                if !flags.contains(ch) {
                    return checker(cmd.clone(), mp, ch);
                }
            }
        }
        true
    }

    pub fn clear_terminal() {
        clearscreen::clear().expect("Failed to clear terminal");
    }
}

pub mod commands;
pub mod lexer;

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
    use crate::commands::ls::exec_ls;
    use crate::commands::touch::exec_touch;
    use crate::commands::echo::exec_echo;




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
        Clear,
        Touch
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
                "touch" => Some(Commands::Touch),
                _ => None,
            }
        }
    }

    pub fn execute(
        cmd: Commands,
        args: &mut Vec<String>,
        cmd_map: &mut std::collections::HashMap<Commands, String>,
    ) {
        let mut catt = 1;
        match cmd {
            Commands::Rm => {
                if let Err(_) = exec_rm(cmd, args, cmd_map) {
                    return ;
                }
                cmd_map.remove(&Commands::Rm);
            }
            Commands::Cd => exec_cd(cmd, args, cmd_map),
            Commands::Mv => {exec_mv(cmd, args, cmd_map); cmd_map.remove(&Commands::Mv);},
            Commands::Pwd => {exec_pwd(cmd, args, cmd_map)},
            Commands::Mkdir => {exec_mkdir(cmd, args, cmd_map); cmd_map.remove(&Commands::Mkdir);},
            Commands::Cp => {exec_cp(cmd, args, cmd_map); cmd_map.remove(&Commands::Cp);},
            Commands::Exit => {exec_exit(args); cmd_map.remove(&Commands::Exit);},
            Commands::Cat => {let _ = exec_cat(&mut catt, cmd, args, cmd_map); cmd_map.remove(&Commands::Cat);},
            Commands::Clear => {exec_clear(cmd, args, cmd_map); cmd_map.remove(&Commands::Clear);},
            Commands::History => {exec_history(cmd, args, cmd_map); cmd_map.remove(&Commands::History);},
            Commands::Ls => {exec_ls(cmd, args, cmd_map); cmd_map.remove(&Commands::Ls);},
            Commands::Touch => {exec_touch(cmd, args, cmd_map); cmd_map.remove(&Commands::Touch);},
            Commands::Echo => {exec_echo(cmd, args, cmd_map)},
        }
    }

    pub fn detect_flags(
        cmd: Commands,
        args: &mut Vec<String>,
        cmd_map: &mut std::collections::HashMap<Commands, String>
    ) {
        for arg in args.clone() {
            if arg.starts_with('-') && arg.len() > 1 {
                // println!("{} --- ++++", arg);
                for flag in arg[1..].chars() {
                    let value = cmd_map.entry(cmd.clone()).or_insert(flag.to_string());
                    // println!("{} --- ++++", amp);
                    if !value.contains(flag) {
                        value.push(flag);
                    }
                }
            }
        }
        args.retain(|arg| !arg.starts_with('-') || arg == "-");
    }

    pub fn valid_flags(
        cmd: Commands,
        cmd_map: &mut std::collections::HashMap<Commands, String>
    ) -> bool {
        return match cmd {
            Commands::Rm => check(cmd.clone(), cmd_map, "r".to_string()),
            Commands::Mkdir => check(cmd.clone(), cmd_map, String::new()),
            Commands::Cp => check(cmd.clone(), cmd_map, "r".to_string()),
            Commands::Cat => check(cmd.clone(), cmd_map, "n".to_string()),
            Commands::Ls => check(cmd.clone(), cmd_map, "alFr".to_string()),
            Commands::Echo => check(cmd.clone(), cmd_map, String::new()),
            Commands::Clear => check(cmd.clone(), cmd_map, String::new()),
            Commands::Pwd => check(cmd.clone(), cmd_map, String::new()),
            Commands::Cd => check(cmd.clone(), cmd_map, String::new()),
            Commands::Mv => check(cmd.clone(), cmd_map, String::new()),
            Commands::History => check(cmd.clone(), cmd_map, String::from("c")),
            Commands::Touch => check(cmd.clone(), cmd_map, String::new()),
            _ => true,
        } ;
    }

    pub fn checker(
        cmd: Commands,
        cmd_map: &mut std::collections::HashMap<Commands, String>,
        flag: char
    ) -> bool {
        cmd_map.remove(&cmd);
        println!("{:?}: invalid option -- '{}'", cmd, flag);
        false
    }

    pub fn check(
        cmd: Commands,
        cmd_map: &mut std::collections::HashMap<Commands, String>,
        flags: String
    ) -> bool {
        if let Some(f) = cmd_map.get(&cmd) {
            for ch in f.chars() {
                if !flags.contains(ch) {
                    return checker(cmd.clone(), cmd_map, ch);
                }
            }
        }
        true
    }

}

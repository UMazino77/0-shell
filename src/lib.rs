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

    // use crate::commands::echo::exec_echo;



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
        mp: &mut std::collections::HashMap<Commands, String>,
    ) {
        let mut catt = 1;
        match cmd {
            Commands::Rm => {
                if let Err(e) = exec_rm(cmd, args, mp) {
                    println!("Error executing rm: {}", e);
                }
                mp.remove(&Commands::Rm);
            }
            Commands::Cd => exec_cd(cmd, args, mp),
            Commands::Mv => {exec_mv(cmd, args, mp); mp.remove(&Commands::Mv);},
            Commands::Pwd => {exec_pwd(cmd, args); mp.remove(&Commands::Pwd);},
            Commands::Mkdir => {exec_mkdir(cmd, args, mp); mp.remove(&Commands::Mkdir);},
            Commands::Cp => {exec_cp(cmd, args, mp); mp.remove(&Commands::Cp);},
            Commands::Exit => {exec_exit(args); mp.remove(&Commands::Exit);},
            Commands::Cat => {let _ = exec_cat(&mut catt, cmd, args, mp); mp.remove(&Commands::Cat);},
            Commands::Clear => {exec_clear(cmd, args, mp); mp.remove(&Commands::Clear);},
            Commands::History => {exec_history(cmd, args, mp); mp.remove(&Commands::History);},
            Commands::Ls => {exec_ls(cmd, args, mp); mp.remove(&Commands::Ls);},
            _ => println!("Command {:?} not implemented yet", cmd),
        }
    }

    pub fn detect_flags(
        cmd: Commands,
        args: &mut Vec<String>,
        mp: &mut std::collections::HashMap<Commands, String>
    ) {
        for arg in args.clone() {
            if arg.starts_with('-') && arg.len() > 1 {

                println!("{} --- ++++", arg);
                for ch in arg[1..].chars() {
                    let amp = mp.entry(cmd.clone()).or_insert(ch.to_string());
                    // println!("{} --- ++++", amp);
                    if !amp.contains(ch) {
                        amp.push(ch);
                    }
                }
            }
        }
        args.retain(|arg| !arg.starts_with('-') || arg == "-");
    }

    pub fn valid_flags(
        cmd: Commands,
        mp: &mut std::collections::HashMap<Commands, String>
    ) -> bool {
        // println!("{:?} --- {:?}", cmd, mp);
        return match cmd {
            Commands::Rm => check(cmd.clone(), mp, "r".to_string()),
            Commands::Mkdir => check(cmd.clone(), mp, "p".to_string()),
            Commands::Cp => check(cmd.clone(), mp, "r".to_string()),
            Commands::Cat => check(cmd.clone(), mp, "n".to_string()),
            Commands::Ls => check(cmd.clone(), mp, "alF".to_string()),
            Commands::Echo => check(cmd.clone(), mp, String::new()),
            Commands::Clear => check(cmd.clone(), mp, String::new()),
            Commands::Pwd => check(cmd.clone(), mp, String::new()),
            Commands::Cd => check(cmd.clone(), mp, String::new()),
            Commands::Mv => check(cmd.clone(), mp, String::new()),
            Commands::History => check(cmd.clone(), mp, String::from("c")),
            _ => true,
        } ;
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

}

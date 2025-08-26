pub mod commands;

pub mod zero {
    use crate::commands::cd::exec_cd;
    use crate::commands::rm::exec_rm;
    use crate::commands::pwd::exec_pwd;
    use crate::commands::mkdir::exec_mkdir;
    use crate::commands::exit::exec_exit;


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
                _ => None,
            }
        }
    }

    pub fn execute(
        cmd: Commands,
        args: &mut [String],
        mp: &mut std::collections::HashMap<Commands, String>,
    ) {
        match cmd {
            Commands::Rm => {
                if let Err(e) = exec_rm(cmd, args, mp) {
                    println!("Error executing rm: {}", e);
                }
            } ,
            Commands::Cd =>  exec_cd(cmd, args, mp) ,
            Commands::Pwd => exec_pwd(cmd, args),
            Commands::Mkdir => exec_mkdir(cmd, args, mp),
            Commands::Exit => exec_exit(args),
            _ => println!("Command {:?} not implemented yet", cmd),
        }
    }

    pub fn detect_flags(
        cmd: Commands,
        args: &[String],
        mp: &mut std::collections::HashMap<Commands, String>,
    ) {
        for arg in args {
            if arg.starts_with('-') {
                for ch in arg[1..].chars() {
                    mp.entry(cmd.clone()).or_insert(ch.to_string()).push(ch);
                }
            }
        }
    }

    pub fn valid_flags(cmd : Commands, mp : &mut std::collections::HashMap<Commands, String>)-> bool
        {
            // println!("{:?} --- {:?}", cmd, mp);
        match cmd {
            Commands::Rm => {
                if let Some(flag) = mp.get(&cmd) {
                    for ch in flag.chars() {
                        match ch {
                            'r' => {},
                            _ => {
                                return checker(cmd.clone(), mp, ch);
                            }
                        }
                    }
                }
            },
            Commands::Mkdir => {
                if let Some(flag) = mp.get(&cmd) {
                    for ch in flag.chars() {
                        match ch {
                            'p' => {},
                            _ => {
                                return checker(cmd.clone(), mp, ch);
                            }
                        }
                    }
                }
            },
            _ => {}
        }
        true
    }

    pub fn checker(cmd : Commands, mp : &mut std::collections::HashMap<Commands, String>, flag : char)->bool{
        mp.remove(&cmd);
        println!("{:?}: invalid option -- '{}'", cmd, flag);
        false
    }
}
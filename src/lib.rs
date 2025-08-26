pub mod commands;

pub mod zero {
    use crate::commands::cd::exec_cd;
    use crate::commands::rm::exec_rm;

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
                    match ch {
                        'r' => {
                            mp.insert(cmd.clone(), "r".to_owned());
                        }
                        _ => println!("Error"),
                    }
                }
            } 
        }
    }
}
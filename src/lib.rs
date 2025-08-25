pub mod zero {
    use std::fs;
    use std::io;  
    use std::path::Path;

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
        args: &[String],
        mp: &mut std::collections::HashMap<Commands, String>,
    ) {
        match cmd {
            Commands::Rm => {
                if let Err(e) = exec_rm(cmd, args, mp) {
                    println!("Error executing rm: {}", e);
                }
            }
            _ => println!("Command {:?} not implemented yet", cmd),
        }
    }

    pub fn exec_rm(
        cmd: Commands,
        args: &[String],
        mp: &mut std::collections::HashMap<Commands, String>,
    ) -> io::Result<()> {
        detect_flags(cmd.clone(), args, mp);

        for i in args {
            if i.starts_with('-') {
                continue;
            }
            
            let path_str = format!("./{}", i);
            let path = Path::new(&path_str);

            if !path.exists() {
                println!("rm: cannot remove '{}': No such file or directory", i);
                continue;
            }

            let metadata = fs::metadata(path)?;

            if metadata.is_dir() {
                if mp.contains_key(&cmd) && mp.get(&cmd) == Some(&"r".to_string()) {
                    fs::remove_dir_all(path)?;
                } else {
                    println!("rm: cannot remove '{}': Is a directory", i);
                }
            } else if metadata.is_file() {
                fs::remove_file(path)?;
            }
        }
        Ok(())
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
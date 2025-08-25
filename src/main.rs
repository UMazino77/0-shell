use shell::zero::*;
use std::{io};
use std::collections::HashMap;

fn main() {
    let mut mp = HashMap::new();
    loop {
        let mut ar = String::new();

        io::stdin()
            .read_line(&mut ar)
            .expect("Failed to read line");
        let args: Vec<&str> = ar.trim().split(|x:char| x == ';').collect();
        if args.len() < 1 {
            println!("Usage: <command> [args...]");
            // std::process::exit(1);
        }
        let mut b: Vec<Vec<String>> = Vec::new();
        for i in args.iter() {
            let a = i.split(|x: char| x.is_ascii_whitespace()).map(|x| x.to_string()).collect();
            b.push(a);
        }
        for j in b.iter() {
            if j.len() < 1 {
                println!("$");
            }
            match Commands::from_str(&j[0]) {
                Some(cmd) => {
                    execute(cmd, &j[1..], &mut mp);
                }
                None => {
                    println!("$ Unknown command: {}", j[0]);
                }
            }
        }
        println!("{:?}", b);

    }
}

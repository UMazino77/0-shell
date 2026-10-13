# 0-shell 🐚

A minimal Unix-like shell written from scratch in **Rust**.

0-shell reads commands from the terminal, parses them with a custom lexer, and runs built-in implementations of common Unix commands, without relying on the system's own binaries.

## Features

- Interactive prompt showing the current directory
- Custom lexer for parsing user input
- Built-in commands implemented from scratch:

| Command   | Description                              |
|-----------|------------------------------------------|
| `ls`      | List directory contents (`-a`, `-l`, `-F`) |
| `cd`      | Change the current directory             |
| `pwd`     | Print the working directory              |
| `cat`     | Print file contents                      |
| `echo`    | Print text to the terminal               |
| `cp`      | Copy files                               |
| `mv`      | Move or rename files                     |
| `rm`      | Remove files                             |
| `mkdir`   | Create directories                       |
| `touch`   | Create files / update timestamps         |
| `history` | Show previously entered commands         |
| `clear`   | Clear the screen                         |
| `exit`    | Exit the shell                           |

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable) and Cargo
- A Unix-like OS (Linux or macOS)

### Build and run

```bash
git clone https://github.com/UMazino77/0-shell.git
cd 0-shell
cargo run
```

For an optimized build:

```bash
cargo build --release
./target/release/0-shell
```

## Usage

```text
$ ls -l src
total 20
drwxrwxr-x 2 user user 4096 Jan 22 2026 commands
drwxrwxr-x 2 user user 4096 Jan 22 2026 lexer
-rw-rw-r-- 1 user user 5530 Jan 22 2026 lib.rs
-rw-rw-r-- 1 user user 2723 Jan 22 2026 main.rs

$ pwd
/home/user/0-shell
```

## Project Structure

```text
0-shell/
├── Cargo.toml
├── src/
│   ├── main.rs         # Entry point and main loop
│   ├── lib.rs          # Shared logic and command dispatch
│   ├── lexer/          # Input tokenizing and parsing
│   └── commands/       # One module per built-in command
│       ├── cat.rs
│       ├── cd.rs
│       ├── clear.rs
│       ├── cp.rs
│       ├── echo.rs
│       ├── exit.rs
│       ├── history.rs
│       ├── ls.rs
│       ├── mkdir.rs
│       ├── mv.rs
│       ├── pwd.rs
│       ├── rm.rs
│       └── touch.rs
└── push.sh
```

## What I Learned

- How a shell works: read, parse, execute, repeat
- Working with the filesystem and Unix metadata (permissions, owners, timestamps) in Rust
- Structuring a Rust project into modules
- Rust's ownership and borrowing rules in a real project

## Contributing

Feedback, issues, and pull requests are welcome. Feel free to open an issue if you spot a bug or have an idea.

## Author

**Mazino** ([@UMazino77](https://github.com/UMazino77)), software engineering student at Zone01 Oujda.

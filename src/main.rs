#[cfg(windows)]
use std::path::Path;
use std::{
    env,
    fs,
    io::{self, Write},
    path::PathBuf,
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn main() {
    let mut shell = Shell::default();

    while shell.state == State::Continue {
        // Print Prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read Input
        let mut input = String::default();
        io::stdin().read_line(&mut input).unwrap();

        match Command::parse(&input) {
            Ok(cmd) => {
                shell.state = cmd.execute();
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}

#[derive(PartialEq, Default)]
enum State {
    #[default]
    Continue,
    Stop,
}

#[derive(Default)]
struct Shell {
    state: State,
}

enum Command {
    Builtin(BuiltinCommand),
    External(String, Vec<String>),
    NoOp,
}

enum BuiltinCommand {
    Exit,
    Echo(String),
    Type(String),
}

impl Command {
    fn execute(&self) -> State {
        match self {
            Command::Builtin(builtin_command) => builtin_command.execute(),
            Command::External(cmd, args) => {
                match find_in_path(cmd) {
                    Some(path) => {
                        let status = std::process::Command::new(path)
                            .args(args)
                            .spawn()
                            .and_then(|mut child| child.wait());

                        if let Err(err) = status {
                            eprintln!("Error executing {}: {}", cmd, err);
                        }
                    }
                    None => println!("{}: command not found", cmd),
                }
                State::Continue
            }
            Command::NoOp => State::Continue,
        }
    }

    fn parse(input: &str) -> anyhow::Result<Self> {
        let mut parts = input.trim().split_ascii_whitespace();

        let first_word = match parts.next() {
            Some(word) => word,
            None => return Ok(Self::NoOp),
        };

        let args: Vec<String> = parts.map(|part| part.to_string()).collect();

        match first_word {
            "exit" => Ok(Self::Builtin(BuiltinCommand::Exit)),
            "echo" => Ok(Self::Builtin(BuiltinCommand::Echo(args.join(" ")))),
            "type" => Ok(Self::Builtin(BuiltinCommand::Type(
                args.get(0).cloned().unwrap_or_default(),
            ))),
            _ => {
                // If it's not a builtin, treat it as an external command
                Ok(Self::External(first_word.to_string(), args))
            }
        }
    }
}

impl BuiltinCommand {
    fn as_str(&self) -> &str {
        match self {
            BuiltinCommand::Exit => "exit",
            BuiltinCommand::Echo(_) => "echo",
            BuiltinCommand::Type(_) => "type",
        }
    }
    fn execute(&self) -> State {
        match self {
            BuiltinCommand::Exit => State::Stop,
            BuiltinCommand::Echo(args) => {
                println!("{}", args);
                State::Continue
            }
            BuiltinCommand::Type(cmd) => {
                match cmd.as_str() {
                    "echo" | "exit" | "type" => {
                        println!("{} is a shell builtin", cmd);
                    }
                    _ => {
                        if let Some(path) = find_in_path(cmd) {
                            println!("{} is {}", cmd, path.display());
                        } else {
                            println!("{}: not found", cmd);
                        }
                    }
                }
                State::Continue
            }
        }
    }
}

fn find_in_path(cmd: &str) -> Option<PathBuf> {
    let path_var = env::var("PATH").ok()?;
    let paths = env::split_paths(&path_var);

    for path in paths {
        let full_path = path.join(cmd);

        #[cfg(unix)]
        {
            if is_executable_unix(&full_path) {
                return Some(full_path);
            }
        }

        #[cfg(windows)]
        {
            if let Some(p) = find_executable_windows(&full_path) {
                return Some(p);
            }
        }
    }
    None
}

#[cfg(unix)]
fn is_executable_unix(path: &PathBuf) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(windows)]
fn find_executable_windows(path: &Path) -> Option<PathBuf> {
    if path.is_file() {
        return Some(path.to_path_buf());
    }

    let pathext = env::var("PATHEXT").unwrap_or_else(|_| ".EXE;.BAT;.CMD".to_string());

    for ext in env::split_paths(&pathext) {
        if let Some(ext_str) = ext.to_str() {
            let mut with_ext = path.to_path_buf();
            with_ext.set_extension(ext_str.trim_start_matches('.'));

            if with_ext.is_file() {
                return Some(with_ext);
            }
        }
    }
    None
}

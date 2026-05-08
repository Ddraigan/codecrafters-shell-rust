use std::io::{self, Write};

fn main() {
    let mut shell = Shell::default();

    while shell.state == State::Continue {
        // Print Prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read Input
        let mut input = String::default();
        io::stdin().read_line(&mut input).unwrap();

        match BuiltinCommand::parse(&input) {
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

enum BuiltinCommand {
    Exit,
    Echo(String),
    Type(String),
    NoOp,
}

#[derive(Default)]
struct Shell {
    state: State,
}

impl BuiltinCommand {
    fn execute(&self) -> State {
        match self {
            BuiltinCommand::Exit => State::Stop,
            BuiltinCommand::Echo(args) => {
                println!("{}", args);
                State::Continue
            }
            BuiltinCommand::Type(cmd) => {
                if matches!(cmd.as_str(), "echo" | "exit" | "type") {
                    println!("{} is a shell builtin", cmd);
                } else {
                    println!("{}: not found", cmd)
                }
                State::Continue
            }
            BuiltinCommand::NoOp => State::Continue,
        }
    }

    fn parse(command: &str) -> anyhow::Result<BuiltinCommand> {
        let mut parts = command.trim().split_ascii_whitespace();

        let first_word = match parts.next() {
            Some(word) => word,
            None => return Ok(Self::NoOp),
        };

        match first_word {
            "exit" => Ok(Self::Exit),
            "echo" => {
                let remainder: Vec<&str> = parts.collect();
                Ok(Self::Echo(remainder.join(" ")))
            }
            "type" => {
                let arg = parts.next().unwrap_or_default();
                Ok(Self::Type(arg.to_string()))
            }
            _ => Err(anyhow::anyhow!("{}: command not found", first_word)),
        }
    }
}

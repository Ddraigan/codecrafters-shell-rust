use std::io::{self, Write};

enum BuiltinCommand {
    Exit,
    Echo(String),
}

impl BuiltinCommand {
    fn execute(command: &str) -> anyhow::Result<BuiltinCommand> {
        let command = command.trim();
        if let Some(args) = command.strip_prefix("echo ") {
            return Ok(BuiltinCommand::Echo(args.into()));
        }

        if command == "exit" {
            return Ok(BuiltinCommand::Exit);
        }

        Err(anyhow::anyhow!("{}: command not found", command))
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::default();
        io::stdin().read_line(&mut command).unwrap();
        match BuiltinCommand::execute(&command) {
            Ok(com) => match com {
                BuiltinCommand::Exit => break,
                BuiltinCommand::Echo(args) => println!("{}", args),
            },
            Err(_) => {
                println!("{}: command not found", command.trim());
            }
        };
    }
}

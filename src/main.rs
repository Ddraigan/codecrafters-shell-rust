use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut command = String::default();
    io::stdin().read_line(&mut command).unwrap();
    println!("{}: command not found", command.trim());
}

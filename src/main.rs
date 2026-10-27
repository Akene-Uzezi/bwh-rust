use std::io;

use bwh_rust::repl;

fn main() -> io::Result<()> {
    println!("Hello! This is the badwhipp language implemented in rust");
    println!("Feel free to type in commands");

    let stdin = io::stdin();
    let stdout = io::stdout();

    let reader = stdin.lock();
    let writer = stdout.lock();

    repl::start(reader, writer)?;

    Ok(())
}

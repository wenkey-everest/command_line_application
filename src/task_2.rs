use std::io;
use std::io::Write;

pub fn task_2() -> io::Result<()> {
    let mut input = String::new();

    print!("Enter a string: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input)?;

    println!("echo ' {} '", input.trim());
    Ok(())
}

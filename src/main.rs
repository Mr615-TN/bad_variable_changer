use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
mod modifyfile;

fn filelinebyline(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    println!("Reading {} line by line: ", file_path);
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("You must provide at least one file path. ");
        return Ok(());
    }
    for file_path in &args[1..] {
        filelinebyline(file_path)?;
    }
    Ok(())
}

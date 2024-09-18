use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file = File::open("./*.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}

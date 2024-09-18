use std::fs::{File, OpenOptions};
use std::fs::{self, BufRead, BufReader, Write};

pub fn appendfile(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;
    writeln!(file, "{}", content);
    Ok(())
}

pub fn prependfile(file_path: &str, content: &str) -> io::Result<()> {
    let og_content = read_file(file_path)?;
    let mut file = File::create(file_path)?;
    writeln!(file, "{}", content)?;
    write!(file, "{}", og_content)?;
    Ok(())
}


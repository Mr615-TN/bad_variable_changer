use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

pub fn appendfile(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

pub fn prependfile(file_path: &str, content: &str) -> io::Result<()> {
    let og_content = read_file(file_path)?;
    let mut file = File::create(file_path)?;
    writeln!(file, "{}", content)?;
    write!(file, "{}", og_content)?;
    Ok(())
}

pub fn read_file(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }
    Ok(content)
}

pub fn write_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

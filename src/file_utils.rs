use std::fs;
use std::io::{Read, BufReader};

pub fn read_pem_file(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let pem = pem::parse(content)?;
    Ok(pem.contents().to_vec())
}

pub fn read_file_content(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

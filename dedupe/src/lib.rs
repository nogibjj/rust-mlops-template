//walks a filesystem and finds duplicate files

use std::collections::HashMap;
use std::error::Error;
use walkdir::WalkDir;

pub fn walk(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }
    Ok(files)
}

// Create a checksum of each file and store in a HashMap if the checksum already exists, add the file to the vector of files with that checksum
pub fn checksum(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let mut checksums = HashMap::new();
    for file in files {
        let checksum = md5::compute(std::fs::read(&file)?);
        let checksum = format!("{:x}", checksum);
        checksums
            .entry(checksum)
            .or_insert_with(Vec::new)
            .push(file);
    }
    Ok(checksums)
}

// Find all the files with more than one entry in the HashMap
pub fn find_duplicates(checksums: HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut duplicates = Vec::new();
    for (_checksum, files) in checksums {
        if files.len() > 1 {
            duplicates.push(files);
        }
    }
    duplicates
}

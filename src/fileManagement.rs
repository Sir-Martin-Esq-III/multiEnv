use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Write},
    result,
};

pub struct FileManager {
    pub file_path: String,
}

impl FileManager {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path: file_path,
        }
    }

    fn write_contents(&self, contents: Vec<String>) -> result::Result<usize, std::io::Error> {
        let path = format!("{}/.envrc", &self.file_path);
        let mut file = File::create(path).unwrap();
        file.write(contents.join("\n").as_bytes())
    }

    fn get_file_lines(&self) -> BufReader<File> {
        let path = format!("{}/.envrc", &self.file_path);
        let mut file = File::open(&path).unwrap();
        BufReader::new(file)
    }

    fn create_pattern(line_value: &str) -> regex::Regex {
        let pattern = format!(
            r"{}",
            line_value.split_inclusive("=").collect::<Vec<&str>>()[0]
        );
        let r = Regex::new(&pattern);
        match r {
            Ok(r) => r,
            _ => panic!(),
        }
    }

    pub fn add_vals_to_file(&self, values_to_add: Vec<String>) -> Result<&'static str, Error> {
        let mut final_contents = values_to_add;
        let file_buffer = FileManager::get_file_lines(&self);
        for line in file_buffer.lines() {
            match line {
                Ok(line_value) => {
                    if line_value == "" {
                        final_contents.push("".to_string());
                        continue;
                    }
                    let pattern = FileManager::create_pattern(&line_value);
                    if !final_contents.iter().any(|value| pattern.is_match(value)) {
                        final_contents.push(line_value)
                    }
                }
                Err(err) => println!("{}", err),
            }
        }
        FileManager::write_contents(&self, final_contents).unwrap();
        Ok("okay")
    }

    pub fn remove_vals_from_file(&self, values_to_rem: Vec<String>) -> Result<&'static str, Error> {
        let mut final_contents = Vec::new();
        let file_buffer = FileManager::get_file_lines(&self);
        for line in file_buffer.lines() {
            match line {
                Ok(line_value) => {
                    if line_value == "" {
                        final_contents.push("".to_string());
                        continue;
                    }
                    let pattern = FileManager::create_pattern(&line_value);
                    if !values_to_rem.iter().any(|value| pattern.is_match(value)) {
                        final_contents.push(line_value)
                    }
                }
                Err(err) => println!("{}", err),
            }
        }
        FileManager::write_contents(&self, final_contents).unwrap();
        Ok("okay")
    }
}

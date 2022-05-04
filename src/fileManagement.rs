use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Write},
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

    pub fn write_to_file(&self, values_to_add: &mut Vec<String>) -> Result<&'static str, Error> {
        //Move this out of here
        let env_file_final_contents = values_to_add;

        let path = format!("{}/.envrc", &self.file_path);
        let mut file = File::open(&path)?;
        let file_buffer = BufReader::new(file);
        for line in file_buffer.lines() {
            match line {
                Ok(line_value) => {
                    let pattern = format!(
                        r"{}",
                        line_value.split_inclusive("=").collect::<Vec<&str>>()[0]
                    );
                    let r = Regex::new(&pattern).unwrap();
                    if !env_file_final_contents
                        .iter()
                        .any(|value| r.is_match(value))
                    {
                        env_file_final_contents.push(line_value)
                    }
                }
                Err(err) => println!("{}", err),
            }
        }
        file = File::create(path)?;
        file.write(env_file_final_contents.join("\n").as_bytes())?;
        Ok("File successfully updated")
    }
}

use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, IoSlice, Write},
};

pub struct FileManager {
    pub file_path: String,
    file_content: String,
}

impl FileManager {
    pub fn New(file_path: String) -> Self {
        Self {
            file_path: file_path,
            file_content: "fileContent".to_string(),
        }
    }

    fn get_exisiting_file_content(self) -> Result<Self, Error> {
        let file = File::open(&self.file_path)?;
        let mut bufReader = BufReader::new(file);
        let mut line = String::new();
        bufReader.read_line(&mut line)?;

        return Ok(self);
    }

    pub fn write_to_file(&self, values_to_add: &Vec<String>) -> Result<&'static str, Error> {
        //Move this out of here
        let mut env_file_final_contents = values_to_add
            .clone()
            .iter()
            .map(|value| format!("export {}", value))
            .collect::<Vec<String>>();

        let path = format!("{}/.envrc", &self.file_path);
        let mut file = File::open(format!("{}/.envrc", &self.file_path))?;
        let file_buffer = BufReader::new(file);
        for line in file_buffer.lines() {
            match line {
                Ok(line_value) => {
                    // println!("{}", line_value);
                    // format!("(?<=export )(.*)(?= \=)")
                    let pattern = format!(r"{}", line_value.split("=").collect::<Vec<&str>>()[0]);
                    let r = Regex::new(&pattern).unwrap();
                    // println!("{:?} {:?}", r.is_match(&line_value), line_value);
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
        Ok("TODO")
    }
}

use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
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

    pub fn write_to_file(&self, values_to_add: String) -> Result<&'static str, Error> {
        println!("here");
        Ok("TODO")
    }
}

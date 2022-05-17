use clap::*;
use serde::{de, Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};

mod fileManagement;

#[derive(Deserialize, Serialize, Debug)]
struct MultiEnvrc {
    folder_paths: Vec<String>,
}

//TODO:Better error handling
//TODO: Move away from relative paths

const MULTI_ENVRC_FILE_PATH: &str = "./multiEnvrc.json";

fn deserialize_config<T: de::DeserializeOwned>(multi_env_rc_path: &str) -> Result<T> {
    let file = File::open(multi_env_rc_path)?;
    let reader = BufReader::new(file);
    let mut v: T = serde_json::from_reader(reader).unwrap();
    Ok(v)
}

fn serialize_config<T: serde::Serialize>(
    file_path: &str,
    content: T,
) -> Result<&'static str, &'static str> {
    let serialised_json = serde_json::to_string_pretty(&content).unwrap();
    let mut file = File::create(file_path).unwrap();
    let file_write_successful = file.write_all(serialised_json.as_bytes());
    match file_write_successful {
        Ok(_) => Ok("Added path"),
        Err(_) => Err("Failed to add path to config file"),
    }
}

fn add(new_path: String) -> Result<&'static str, &'static str> {
    let all_paths = deserialize_config::<MultiEnvrc>(MULTI_ENVRC_FILE_PATH);
    match all_paths {
        Ok(mut paths) => {
            if paths.folder_paths.contains(&new_path) {
                return Ok("Path already exists");
            }
            paths.folder_paths.push(new_path);
            let file_write_successful =
                serialize_config::<MultiEnvrc>(MULTI_ENVRC_FILE_PATH, paths);
            match file_write_successful {
                Ok(_) => Ok("Added path"),
                Err(_) => Err("Failed to add path to config file"),
            }
        }
        Err(_) => Err("Failed to read from config file"),
    }
}

fn remove(path_to_remove: String) -> Result<&'static str, &'static str> {
    let mut all_paths = deserialize_config::<MultiEnvrc>(MULTI_ENVRC_FILE_PATH);
    match all_paths {
        Ok(mut paths) => {
            if paths.folder_paths.contains(&path_to_remove) == false {
                return Ok("Path doesnt exist in your config");
            }
            paths.folder_paths.retain(|path| *path != path_to_remove);
            let file_write_successful =
                serialize_config::<MultiEnvrc>(MULTI_ENVRC_FILE_PATH, paths);
            match file_write_successful {
                Ok(_) => Ok("Removed path"),
                Err(_) => Err("Failed to add path to config file"),
            }
        }
        Err(_) => Err("Failed to read from config file"),
    }
}

fn push(values: &Vec<String>) -> Result<&'static str> {
    let all_paths = deserialize_config::<MultiEnvrc>(MULTI_ENVRC_FILE_PATH)?;
    let format_values = values
        .iter()
        .map(|keys| format!("export {}", keys))
        .collect::<Vec<_>>();
    for path in all_paths.folder_paths.iter() {
        fileManagement::FileManager::new(path.to_string())
            .add_vals_to_file(format_values.clone())?;
    }
    return Ok("added or updated env keys");
}

fn delete(values: &Vec<String>) -> Result<&'static str> {
    let all_paths = deserialize_config::<MultiEnvrc>(MULTI_ENVRC_FILE_PATH)?;
    let format_values = values
        .iter()
        .map(|keys| format!("export {}=", keys))
        .collect::<Vec<_>>();
    for path in all_paths.folder_paths.iter() {
        fileManagement::FileManager::new(path.to_string())
            .remove_vals_from_file(format_values.clone())?;
    }
    return Ok("Removed env keys");
}

/// Simple program to update environment variables in multiple places
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///Add a path to your projects path list
    #[clap(short, long)]
    add: Option<String>,

    ///Remove a path from your project path list
    #[clap(short, long)]
    remove: Option<String>,

    ///Push a key(s) to your env files
    #[clap(short, long, multiple_values = true)]
    push: Option<Vec<String>>,

    ///Deletes a key(s) from your env files
    #[clap(short, long, multiple_values = true)]
    delete: Option<Vec<String>>,
}

fn main() {
    let c = Args::parse();

    match c.add {
        Some(value) => {
            let res = add(value);
            match res {
                Ok(val) => println!("{}", val),
                Err(error) => println!("{}", error),
            }
        }
        None => (),
    }
    match c.remove {
        Some(value) => {
            let res = remove(value);
            match res {
                Ok(val) => println!("{}", val),
                Err(error) => println!("{}", error),
            }
        }
        None => (),
    }
    match c.push {
        Some(value) => {
            let res = push(&value);
            match res {
                Ok(val) => println!("{}", val),
                Err(error) => println!("Error trying to push updated values: -> {}", error),
            }
        }
        None => (),
    }
    match c.delete {
        Some(value) => {
            let res = delete(&value);
            match res {
                Ok(val) => println!("{}", val),
                Err(error) => println!("Error trying to delete values: -> {}", error),
            }
        }
        None => (),
    }
}

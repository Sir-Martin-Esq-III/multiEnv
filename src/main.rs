use clap::*;
use serde::{de, Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};

mod fileManagement;

#[derive(Deserialize, Serialize, Debug)]
struct MultiEnvrc {
    folder_paths: Vec<String>,
}

fn get_all_paths<T: de::DeserializeOwned>(multi_env_rc_path: &str) -> Result<T> {
    let file = File::open(multi_env_rc_path)?;
    let reader = BufReader::new(file);
    let mut v: T = serde_json::from_reader(reader).unwrap();
    Ok(v)
}

fn add(new_path: String) -> Result<&'static str> {
    let path_file = "../../multiEnvrc.json";
    let mut all_paths = get_all_paths::<MultiEnvrc>(path_file)?;
    if all_paths.folder_paths.contains(&new_path) {
        return Ok("Path already exists");
    }

    all_paths.folder_paths.push(new_path);
    let string = serde_json::to_string_pretty(&all_paths);
    match string {
        Err(err) => {
            println!("{:?}", err);
        }
        Ok(val) => {
            let mut file = File::create(path_file)?;
            file.write_all(val.as_bytes())?;
        }
    }
    return Ok("Added path");
}

fn remove(path_to_remove: String) -> Result<&'static str> {
    let path_file = "../../multiEnvrc.json";
    let mut all_paths = get_all_paths::<MultiEnvrc>(path_file)?;

    if all_paths.folder_paths.contains(&path_to_remove) == false {
        return Ok("Path doesnt exist");
    }

    all_paths
        .folder_paths
        .retain(|path| *path != path_to_remove);

    let serialised_json = serde_json::to_string_pretty(&all_paths);
    match serialised_json {
        Err(err) => {
            println!("{:?}", err);
        }
        Ok(val) => {
            let mut file = File::create(path_file)?;
            file.write_all(val.as_bytes())?;
        }
    }
    return Ok("Path Deleted");
}

fn push(values: &Vec<String>) -> Result<&'static str> {
    let path_file = "../../multiEnvrc.json";
    let all_paths = get_all_paths::<MultiEnvrc>(path_file)?;
    let format_values = values
        .iter()
        .map(|keys| format!("export {}", keys))
        .collect::<Vec<_>>();
    for path in all_paths.folder_paths.iter() {
        fileManagement::FileManager::new(path.to_string())
            .write_to_file(&mut format_values.clone())?;
    }
    return Ok("added or updated env keys");
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

    ///Push a change to your env files
    #[clap(short, long, multiple_values = true)]
    push: Option<Vec<String>>,
}

fn main() {
    let c = Args::parse();

    match c.add {
        Some(value) => {
            let res = add(value);
            match res {
                Ok(val) => println!("{:?}", val),
                Err(error) => println!("{:?}", error),
            }
        }
        None => (),
    }
    match c.remove {
        Some(value) => {
            let res = remove(value);
            println!("res {:?}", res);
        }
        None => (),
    }
    match c.push {
        Some(value) => {
            let res = push(&value);
            println!("res {:?}", res);
        }
        None => (),
    }
}

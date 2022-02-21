use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::todo_list;

const FILE_NAME: &str = ".config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Notion {
    list_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub source: String,
    pub notion: Notion
}

impl Config {
    pub fn load() -> Config {
        let mut file = Config::file();
        let mut json = String::new();
        file.read_to_string(&mut json)
            .expect("Failed to read file");
        serde_json::from_str(&json).unwrap()
    }

    pub fn save(&self) -> Config {
        let mut file = Config::file();
        let stringified = serde_json::to_string(&self).unwrap();
        file.write_all(stringified.as_bytes())
            .expect("Failed to save config");
        Config::load()
    }

    pub fn inst() -> Config {
        Config {
            source: String::from("file"),
            notion: Notion { list_id: String::from("") }
        }
    }

    pub fn file() -> File {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(Config::file_name())
            .unwrap();

        file
    }

    fn data_dir() -> PathBuf {
        Path::new(&dirs::data_dir().unwrap()).join(todo_list::APP_NAME)
    }

    fn file_name() -> PathBuf {
        Path::new(&Config::data_dir()).join(FILE_NAME)
    }
}

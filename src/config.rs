use std::io;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::todo_list;

const FILE_NAME: &str = ".config.json";

#[derive(Serialize, Deserialize, Debug)]
pub enum ConfigSource {
    File,
    Notion
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notion {
    list_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    source: String,
    notion: Notion
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
        Config::delete_file();
        let mut file = Config::file();
        let stringified = serde_json::to_string(&self).unwrap();
        file.write_all(stringified.as_bytes())
            .expect("Failed to save config");
        Config::load()
    }

    pub fn inst() -> Config {
        Config {
            source: String::from(""),
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

    fn set_source(&self, config_source: ConfigSource) {
        let source = match config_source {
            ConfigSource::File => String::from("file"),
            ConfigSource::Notion => String::from("notion")
        };

        let config = match config_source {
            ConfigSource::File => {
                Config {
                    source,
                    notion: Notion { list_id: String::from("") }
                }
            }
            ConfigSource::Notion => {
                Config {
                    source,
                    notion: Notion { list_id: String::from("112233") }
                }
            }
        };

        config.save();
    }

    pub fn configure(&self) {
        println!("Do you want to save your todos to a file or Notion?");
        println!("1: File");
        println!("2: Notion");

        let mut source = String::new();
        io::stdin().read_line(&mut source).expect("Failed to read input");
        let choice: u8 = source.trim().parse().unwrap();

        match choice {
            1 => self.set_source(ConfigSource::File),
            2 => self.set_source(ConfigSource::Notion),
            _ => panic!("Invalid choice")
        }
    }

    fn data_dir() -> PathBuf {
        Path::new(&dirs::data_dir().unwrap()).join(todo_list::APP_NAME)
    }

    fn file_name() -> PathBuf {
        Path::new(&Config::data_dir()).join(FILE_NAME)
    }

    fn delete_file() {
        std::fs::remove_file(Config::file_name()).expect("Failed to delete config file");
    }
}

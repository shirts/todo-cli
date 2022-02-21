use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::{File, OpenOptions};

const APP_NAME: &str = "todo-cli";
const FILE_NAME: &str = "todo.json";
const BACKUP_FILE_NAME: &str = ".todo.json.bak";

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub items: Vec<String>,
}

impl TodoList {
    // Deserialize contents of todo file into a TodoList
    pub fn load() -> TodoList {
        let mut file = TodoList::file();
        let mut json = String::new();
        file.read_to_string(&mut json)
            .expect("Failed to read file contents");
        serde_json::from_str(&json).unwrap()
    }

    // An empty TodoList
    pub fn inst() -> TodoList {
        TodoList { items: Vec::new() }
    }

    // Add a todo item
    pub fn add(&self, item: String) -> TodoList {
        let mut items = self.items.clone();
        items.push(item);
        let todo_list = TodoList { items };
        todo_list.save();
        TodoList::load()
    }

    // Display the items todo
    pub fn show(&self) {
        println!("{}", self);
    }

    // Complete a todo item
    pub fn complete(&self, index: usize) -> TodoList {
        let mut items = self.items.clone();
        items.remove(index);
        TodoList::backup();
        TodoList::delete_file();
        let todo_list = TodoList { items };
        todo_list.save()
    }

    // Reset the todo list
    pub fn clear(&self) -> () {
        TodoList::delete_file();
        TodoList::file();
    }

    // Open the todo file
    pub fn file() -> File {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(TodoList::file_name())
            .unwrap();

        file
    }

    // Save todo list
    pub fn save(&self) -> TodoList {
        let mut file = TodoList::file();
        let stringified = serde_json::to_string(&self).unwrap();
        file.write_all(stringified.as_bytes())
            .expect("Failed to save");
        TodoList::load()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // The place to save the todo file
    pub fn data_dir() -> PathBuf {
        Path::new(&dirs::data_dir().unwrap()).join(APP_NAME)
    }

    pub fn file_name() -> PathBuf {
        Path::new(&TodoList::data_dir()).join(FILE_NAME)
    }

    fn backup_file_name() -> PathBuf {
        Path::new(&TodoList::data_dir()).join(BACKUP_FILE_NAME)
    }

    // Create a backup of the todo file
    fn backup() {
        std::fs::copy(TodoList::file_name(), TodoList::backup_file_name())
            .expect("Failed to backup");
    }

    // Delete the todo file
    fn delete_file() {
        std::fs::remove_file(TodoList::file_name()).expect("Failed to delete todo file");
    }
}

impl std::fmt::Display for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "There are no todo items").expect("Failed to display");
        } else {
            for (index, item) in self.items.iter().enumerate() {
                write!(f, "{}: {}\n", index + 1, item).expect("Failed to display");
            }
        }

        Ok(())
    }
}

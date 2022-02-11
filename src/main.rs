use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

extern crate dirs;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

const APP_NAME: &str = "todo-cli";
const FILE_NAME: &str = "todo.json";
const BACKUP_FILE_NAME: &str = ".todo.json.bak";

#[derive(Debug)]
enum Action {
    Add,
    Clear,
    Complete,
    Show,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    items: Vec<String>,
}

impl TodoList {
    // Deserialize contents of todo file into a TodoList
    fn load() -> TodoList {
        let mut file = TodoList::file();
        let mut json = String::new();
        file.read_to_string(&mut json)
            .expect("Failed to read file contents");
        serde_json::from_str(&json).unwrap()
    }

    // The place to save the todo file
    fn data_dir() -> PathBuf {
        Path::new(&dirs::data_dir().unwrap()).join(APP_NAME)
    }

    fn file_name() -> PathBuf {
        Path::new(&TodoList::data_dir()).join(FILE_NAME)
    }

    fn backup_file_name() -> PathBuf {
        Path::new(&TodoList::data_dir()).join(BACKUP_FILE_NAME)
    }

    // Open the todo file
    fn file() -> File {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(TodoList::file_name())
            .unwrap();

        file
    }

    // An empty TodoList
    fn inst() -> TodoList {
        TodoList { items: Vec::new() }
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

    // Save todo list
    fn save(&self) -> TodoList {
        let mut file = TodoList::file();
        let stringified = serde_json::to_string(&self).unwrap();
        file.write_all(stringified.as_bytes())
            .expect("Failed to save");
        TodoList::load()
    }

    // Add a todo item
    fn add(&self, item: String) -> TodoList {
        let mut items = self.items.clone();
        items.push(item);
        let todo_list = TodoList { items: items };
        todo_list.save();
        TodoList::load()
    }

    // Display the items todo
    fn show(&self) {
        println!("{}", self);
    }

    // Complete a todo item
    fn complete(&self, index: usize) -> TodoList {
        let mut items = self.items.clone();
        items.remove(index);
        TodoList::backup();
        TodoList::delete_file();
        let todo_list = TodoList { items: items };
        todo_list.save()
    }

    // Reset the todo list
    fn clear(&self) -> () {
        TodoList::delete_file();
        TodoList::file();
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
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

// convert user text to enum
fn convert_to_action(action_string: &str) -> Option<Action> {
    match action_string {
        "a" => Some(Action::Add),
        "add" => Some(Action::Add),
        "c" => Some(Action::Complete),
        "complete" => Some(Action::Complete),
        "clear" => Some(Action::Clear),
        "show" => Some(Action::Show),
        _ => None,
    }
}

fn create_todo_file_if_not_exists() -> std::result::Result<(), std::io::Error> {
    // create directory if doesnt exist
    std::fs::create_dir_all(TodoList::data_dir()).expect("Failed to create data directory");

    let file_exists: bool = Path::new(&TodoList::file_name()).exists();

    if file_exists {
        // make sure it has the expected structure
        let mut contents = String::new();
        TodoList::file()
            .read_to_string(&mut contents)
            .expect("Failed to read file contents");
        if contents == String::from("") {
            let todo_list = TodoList::inst();
            todo_list.save();
        }

        return Ok(());
    }

    let mut file = TodoList::file();
    let todo_list = TodoList::inst();
    let stringified = serde_json::to_string(&todo_list).unwrap();
    file.write_all(stringified.as_bytes())
}

fn main() {
    create_todo_file_if_not_exists().expect("Failed to create file");

    let todo_list = TodoList::load();

    let args: Vec<String> = env::args().collect();

    let subcommand = String::from(args[1].trim());

    let options = String::from(args[2..].join(" ").trim());

    let action: Option<Action> = convert_to_action(&subcommand);

    match action {
        Some(Action::Add) => {
            todo_list.add(options);
        }
        Some(Action::Clear) => {
            todo_list.clear();
        }
        Some(Action::Complete) => {
            let index = options.trim().parse::<i32>().unwrap() as usize - 1;
            todo_list.complete(index);
        }
        Some(Action::Show) => todo_list.show(),
        None => println!("Action not found"),
    }
}

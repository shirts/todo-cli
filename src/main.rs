use std::path::Path;
use std::{env, process};
use std::io::prelude::*;

extern crate dirs;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use action::Action;
pub mod action;

mod todo_list;
use todo_list::TodoList;

fn create_todo_file_if_not_exists() -> std::result::Result<(), std::io::Error> {
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
    let args: Vec<String> = env::args().collect();
    let subcommand = String::from(args[1].trim());
    let options = String::from(args[2..].join(" ").trim());
    let action: Option<Action> = action::convert_to_action(&subcommand);

    if let None = action {
        println!("Action not found");
        process::exit(1);
    }

    create_todo_file_if_not_exists()
        .expect("Failed to create todo file");

    let todo_list = TodoList::load();

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
        None => ()
    }
}

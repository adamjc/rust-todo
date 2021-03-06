#[allow(unused_imports)]

use serde_derive::Deserialize;
use docopt::Docopt;
use std::fs::OpenOptions;
use std::io::{self, Write, Read};
use std::process;

const USAGE: &'static str = "
TODO

Usage: 
  todo add <title> <description>
  todo list
";

#[derive(Debug, Deserialize)]
struct Args {
  cmd_add: bool,
  cmd_list: bool,
  arg_title: String,
  arg_description: String
}

struct Todos {
  todos: Vec<Todo>
}

#[derive(Debug)]
struct Todo {
  title: String,
  description: String
}

fn main() -> std::io::Result<()> {
  let args: Args = Docopt::new(USAGE)
                    .and_then(|d| d.deserialize())
                    .unwrap_or_else(|e| e.exit());
  
  let mut todos = Todos { todos: vec![] };

  let mut file = OpenOptions::new()
                  .read(true)
                  .write(true)
                  .create(true)
                  .open("todos.md")
                  .unwrap_or_else(|_| process::exit(0));
  
  let mut stringified_todos = String::new();
  file.read_to_string(&mut stringified_todos).unwrap_or_else(|_| process::exit(0));
  let split_todos = stringified_todos.split("## ").into_iter().filter(|x| !x.is_empty());

  for mut stringified_todo in split_todos {
    stringified_todo = stringified_todo.trim_end();
    let mut split_todo = stringified_todo.split("\n").into_iter().filter(|x| !x.is_empty());
    let title = split_todo.next().unwrap_or_else(|| process::exit(0));
    let description = split_todo.next().unwrap_or_else(|| process::exit(0));
    let todo = Todo { title: title.to_string(), description: description.to_string() };
    todos.todos.push(todo)
  }

  if args.cmd_add {
    let mut description;
    if args.arg_description.is_empty() {
      description = String::new();  
      print!("Enter a description: ");
      io::stdout().flush()?;
      io::stdin().read_line(&mut description).unwrap();
    } else {
      description = args.arg_description;
    }

    let todo = Todo { 
      title: args.arg_title,
      description: description
    };

    todos.todos.push(todo);

    println!("Todos are: {:?}", todos.todos);
  }

  Ok(())
}

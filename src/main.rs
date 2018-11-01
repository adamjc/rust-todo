use serde_derive::Deserialize;
use docopt::Docopt;
use std::fs::OpenOptions;
use std::io::{self, Write};

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

  let file = OpenOptions::new()
                  .read(true)
                  .write(true)
                  .create(true)
                  .open("todos.txt");

  let mut todos = Todos { todos: vec![] };

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

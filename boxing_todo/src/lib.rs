mod err;
use std::error::Error;
use std::fs;
pub use crate::err::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let file_content = match fs::read_to_string(path) {
            Ok(ok) => ok,
            Err(err) => {
                return Err(Box::new(ReadErr {
                    child_err: Box::new(err),
                }));
            }
        };

        match json::parse(&file_content) {
            Ok(data) => {
                let mut todolist = TodoList{title: data["title"].to_string(), tasks: Vec::new()};
                if data["tasks"].members().len() == 0 {
                    return Err(Box::new(ParseErr::Empty))
                }
                for task in data["tasks"].members() {
                    let t = Task{
                        id: task["id"].as_u32().unwrap(),
                        description: task["description"].to_string(),
                        level: task["level"].as_u32().unwrap(),
                    };
                    todolist.tasks.push(t);
                }
                return Ok(todolist)
            }
            Err(err) => {
                return Err(Box::new(ParseErr::Malformed(Box::new(err))));
            }
        }
    }
}
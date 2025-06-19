mod err;
use std::error::Error;
use std::fs;

use crate::err::{ParseErr, ReadErr};

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
/*
let parsed = json::parse(&file_content)
.map_err(|e| Box::new(ReadErr { child_err: Some(Box::new(e)) }) as Box<dyn Error>)?;

let title = parsed["title"].as_str().unwrap_or("").to_string();
if title.is_empty() {
return Err(Box::new(ReadErr { child_err: None }));
}
let items = parsed["items"]
.members()
.filter_map(|item| item.as_str().map(|s| s.to_string()))
.collect();
Ok(TodoList { title, items })
}
 */

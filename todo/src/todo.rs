use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;


#[derive(Debug, Serialize, Deserialize)]
enum Schedule {
    NoDeadline,
    Due(DateTime<Utc>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToDo {
    name: String,
    description: Option<String>,
    done: bool,
    schedule: Schedule,
}

impl ToDo {
    pub fn new(
        name: String,
        description: Option<String>,
        due_in_days: Option<i64>,
    ) -> Self {
        let schedule = match due_in_days {
            Some(days) => Schedule::Due(Utc::now() + Duration::days(days)),
            None => Schedule::NoDeadline,
        };

        Self {
            name,
            description,
            done: false,
            schedule,
        }
    }

    pub fn view_todo(&self) {
        println!("todo");
        println!("name: {}", self.name);
        println!("description: {:?}", self.description);
        println!("schedule: {:?}", self.schedule);
    }

    pub fn add(self) {
        let file_path = "todos.json";

        if !Path::new(file_path).exists() {
            fs::write(file_path, "[]").expect("Failed to create file");
        }

        let content =
            fs::read_to_string(file_path).expect("Failed to read file");

        let mut todos: Vec<ToDo> =
            serde_json::from_str(&content).unwrap_or_default();

        todos.push(self);

        let json =
            serde_json::to_string_pretty(&todos).unwrap();

        fs::write(file_path, json)
            .expect("Failed to write file");
    }
}
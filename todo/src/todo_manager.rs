use crate::todo::ToDo;
use std::fs;
use std::path::Path;

pub struct TodoManager {
    file_path: String,
}

impl TodoManager {
    pub fn new(file_path: &str) -> Self {
        if !Path::new(file_path).exists() {
            fs::write(file_path, "[]")
                .expect("Failed to create todo file");
        }

        Self {
            file_path: file_path.to_string(),
        }
    }

    fn load(&self) -> Vec<ToDo> {
        let content = fs::read_to_string(&self.file_path)
            .unwrap_or_else(|_| "[]".to_string());

        serde_json::from_str(&content)
            .unwrap_or_default()
    }

    fn save_all(&self, todos: &[ToDo]) {
        let json = serde_json::to_string_pretty(todos)
            .expect("Failed to serialize todos");

        fs::write(&self.file_path, json)
            .expect("Failed to save todos");
    }

    pub fn add(&self, todo: ToDo) {
        let mut todos = self.load();

        todos.push(todo);

        self.save_all(&todos);
    }

    pub fn get_all(&self) -> Vec<ToDo> {
        self.load()
    }

    pub fn view_all(&self) {
        let todos = self.load();

        if todos.is_empty() {
            println!("No todos found");
            return;
        }

        for todo in &todos {
            todo.view();
        }
    }

    pub fn get_by_id(&self, id: u64) -> Option<ToDo> {
        let todos = self.load();

        todos.into_iter().find(|todo| todo.id == id)
    }

    pub fn update(&self, updated_todo: ToDo) -> bool {
        let mut todos = self.load();

        if let Some(todo) = todos
            .iter_mut()
            .find(|todo| todo.id == updated_todo.id)
        {
            *todo = updated_todo;

            self.save_all(&todos);

            return true;
        }

        false
    }

    pub fn delete(&self, id: u64) -> bool {
        let mut todos = self.load();

        let original_len = todos.len();

        todos.retain(|todo| todo.id != id);

        if todos.len() == original_len {
            return false;
        }

        self.save_all(&todos);

        true
    }

    pub fn mark_done(&self, id: u64) -> bool {
        let mut todos = self.load();

        if let Some(todo) =
            todos.iter_mut().find(|todo| todo.id == id)
        {
            todo.done = true;

            self.save_all(&todos);

            return true;
        }

        false
    }

    pub fn count(&self) -> usize {
        self.load().len()
    }

    pub fn clear(&self) {
        self.save_all(&[]);
    }
}
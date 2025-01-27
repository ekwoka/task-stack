use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            title,
            description: None,
            created_at: Utc::now(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

pub struct TaskStack {
    tasks: Mutex<Vec<Task>>,
}

impl TaskStack {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(Vec::new()),
        }
    }

    pub fn push(&self, task: Task) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task);
    }

    pub fn pop(&self) -> Option<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        if tasks.is_empty() {
            None
        } else {
            Some(tasks.remove(0))
        }
    }

    pub fn first(&self) -> Option<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.first().cloned()
    }

    pub fn size(&self) -> usize {
        let tasks = self.tasks.lock().unwrap();
        tasks.len()
    }

    pub fn find_task_position(&self, task: &Task) -> Option<usize> {
        let tasks = self.tasks.lock().unwrap();
        tasks.iter().position(|t| t.created_at == task.created_at)
    }
}

impl Default for TaskStack {
    fn default() -> Self {
        Self::new()
    }
}

#[tauri::command]
pub fn push_task(
    stack: State<TaskStack>,
    title: String,
    description: Option<String>,
) -> Result<(), String> {
    let task = Task {
        title,
        description,
        created_at: Utc::now(),
    };

    let task_clone = task.clone();
    stack.push(task_clone);
    let position = stack.find_task_position(&task).unwrap_or(0) + 1;
    println!(
        "Task added! It's {} in the queue.",
        if position == 1 {
            "next".to_string()
        } else {
            format!("#{}", position)
        }
    );
    Ok(())
}

#[tauri::command]
pub fn complete_top_task(stack: State<TaskStack>) -> Result<(), String> {
    if let Some(_top_task) = stack.pop() {
        Ok(())
    } else {
        Err("No tasks to complete".to_string())
    }
}

#[tauri::command]
pub fn get_top_task(stack: State<TaskStack>) -> Result<Option<Task>, String> {
    Ok(stack.first())
}

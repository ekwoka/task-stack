use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub title: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Task {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    pub fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at
    }
}

pub struct TaskStack(pub Mutex<Vec<Task>>);

impl TaskStack {
    pub fn new() -> Self {
        Self(Mutex::new(Vec::new()))
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
        created_at: chrono::Utc::now(),
    };

    stack.0.lock().map_err(|e| e.to_string())?.push(task);

    Ok(())
}

#[tauri::command]
pub fn complete_top_task(stack: State<TaskStack>) -> Result<(), String> {
    stack.0.lock().map_err(|e| e.to_string())?.pop();

    Ok(())
}

#[tauri::command]
pub fn get_top_task(stack: State<TaskStack>) -> Result<Option<Task>, String> {
    let tasks = stack.0.lock().map_err(|e| e.to_string())?;
    Ok(tasks.last().cloned())
}

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use crate::types::PageResponse;
use crate::routes;

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
) -> Result<PageResponse, String> {
    let task = Task {
        title,
        description,
        created_at: chrono::Utc::now(),
    };

    let mut tasks = stack.0.lock().map_err(|e| e.to_string())?;
    tasks.push(task);
    let position = tasks.len();
    drop(tasks);

    let base_response = routes::index(stack)?;
    
    Ok(PageResponse::with_notification(
        base_response.html,
        format!("Task added! It's {} in the queue.", 
            if position == 1 { "next".to_string() } else { format!("#{}", position) }
        ),
        "success",
        None,
    ))
}

#[tauri::command]
pub fn complete_top_task(stack: State<TaskStack>) -> Result<PageResponse, String> {
    let mut tasks = stack.0.lock().map_err(|e| e.to_string())?;
    
    if tasks.is_empty() {
        return Err("No tasks to complete".to_string());
    }
    
    tasks.remove(0);
    drop(tasks);
    
    let base_response = routes::index(stack)?;
    
    Ok(PageResponse::with_notification(
        base_response.html,
        "Task completed! Great work! 🎉".to_string(),
        "success",
        None,
    ))
}

#[tauri::command]
pub fn get_top_task(stack: State<TaskStack>) -> Result<Option<Task>, String> {
    let tasks = stack.0.lock().map_err(|e| e.to_string())?;
    Ok(tasks.first().cloned())
}

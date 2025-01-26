use crate::routes;
use crate::types::PageResponse;
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
    pub fn new(title: String) -> Self {
        Self {
            title,
            description: None,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
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

    pub fn push(&self, task: Task) {
        let mut tasks = self.0.lock().unwrap();
        tasks.push(task);
    }

    pub fn pop(&self) -> Option<Task> {
        let mut tasks = self.0.lock().unwrap();
        if tasks.is_empty() {
            None
        } else {
            Some(tasks.remove(0))
        }
    }

    pub fn first(&self) -> Option<Task> {
        let tasks = self.0.lock().unwrap();
        tasks.first().cloned()
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
) -> Result<PageResponse, String> {
    let task = Task {
        title,
        description,
        created_at: chrono::Utc::now(),
    };

    stack.push(task);
    let position = stack.first().map(|_| 1).unwrap_or(0) + 1;

    let base_response = routes::index(stack)?;

    Ok(PageResponse::with_notification(
        base_response.updates.first().unwrap().clone(),
        format!(
            "Task added! It's {} in the queue.",
            if position == 1 {
                "next".to_string()
            } else {
                format!("#{}", position)
            }
        ),
        "success",
        None,
    ))
}

#[tauri::command]
pub fn complete_top_task(stack: State<TaskStack>) -> Result<PageResponse, String> {
    if let Some(_top_task) = stack.pop() {
        let base_response = routes::index(stack)?;

        Ok(PageResponse::with_notification(
            base_response.updates.first().unwrap().clone(),
            "Task completed! Great work! 🎉".to_string(),
            "success",
            None,
        ))
    } else {
        Err("No tasks to complete".to_string())
    }
}

#[tauri::command]
pub fn get_top_task(stack: State<TaskStack>) -> Result<Option<Task>, String> {
    Ok(stack.first())
}

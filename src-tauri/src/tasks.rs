use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use ulid::Ulid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskState {
    Active,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Ulid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub state: TaskState,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: Ulid::new(),
            title,
            description: None,
            created_at: Utc::now(),
            state: TaskState::Active,
            completed_at: None,
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

    pub fn completed(&self) -> bool {
        self.state == TaskState::Completed
    }

    pub fn completed_at(&self) -> Option<&DateTime<Utc>> {
        self.completed_at.as_ref()
    }

    pub fn mark_completed(&mut self) {
        self.state = TaskState::Completed;
        self.completed_at = Some(Utc::now());
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
        tasks.iter()
            .find(|task| task.state == TaskState::Active)
            .cloned()
    }

    pub fn first_task(&self) -> Option<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.first().cloned()
    }

    pub fn size(&self) -> usize {
        let tasks = self.tasks.lock().unwrap();
        tasks.len()
    }

    pub fn find_task_position(&self, task: &Task) -> Option<usize> {
        let tasks = self.tasks.lock().unwrap();
        tasks.iter().position(|t| t.id == task.id)
    }

    pub fn complete_task(&self, id: Ulid) -> Result<Task, String> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            // Find position of first incomplete task
            let first_incomplete = tasks.iter()
                .position(|t| t.state == TaskState::Active);
            
            match first_incomplete {
                Some(first_pos) if pos == first_pos => {
                    let mut task = tasks[pos].clone();
                    task.mark_completed();
                    tasks[pos] = task.clone();
                    Ok(task)
                }
                Some(_) => Err("Can only complete the first incomplete task".to_string()),
                None => Err("No incomplete tasks remaining".to_string()),
            }
        } else {
            Err("Task not found".to_string())
        }
    }

    pub fn move_to_end(&self, id: Ulid) -> Result<(), String> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            let task = tasks.remove(pos);
            tasks.push(task);
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
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
        id: Ulid::new(),
        title,
        description,
        created_at: Utc::now(),
        state: TaskState::Active,
        completed_at: None,
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
pub fn get_top_task(stack: State<TaskStack>) -> Result<Option<Task>, String> {
    Ok(stack.first())
}

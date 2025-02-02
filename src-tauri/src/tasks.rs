use crate::database;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
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
    db: libsql::Database,
}

impl TaskStack {
    pub async fn new(db: libsql::Database) -> Self {
        let tasks = database::get_all_tasks(&db)
            .await
            .expect("Failed to load tasks")
            .into_iter()
            .map(|(task, _)| task)
            .collect();
        Self {
            tasks: Mutex::new(tasks),
            db,
        }
    }

    pub async fn push(&self, task: Task) {
        let position = {
            let tasks = self.tasks.lock().unwrap();
            (tasks.len() + 1) as i64
        };
        crate::database::insert_task(&self.db, &task, position)
            .await
            .expect("Failed to insert task");
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task);
    }

    pub async fn pop(&self) -> Option<Task> {
        let task = self.tasks.lock().unwrap().pop();
        if let Some(task) = task {
            crate::database::delete_task(&self.db, &task.id)
                .await
                .expect("Failed to delete task");
            Some(task)
        } else {
            None
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
        tasks.iter().position(|t| t.id == task.id)
    }

    pub async fn complete_task(&self, id: Ulid) -> Result<Task, String> {
        let task_update = {
            let mut tasks = self.tasks.lock().unwrap();
            if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                let mut task = tasks[pos].clone();
                task.state = TaskState::Completed;
                task.completed_at = Some(Utc::now());
                tasks[pos] = task.clone();
                Some(task)
            } else {
                None
            }
        };

        if let Some(task) = task_update {
            crate::database::update_task_state(
                &self.db,
                &id,
                TaskState::Completed,
                Some(Utc::now()),
            )
            .await
            .map_err(|e| e.to_string())?;
            Ok(task)
        } else {
            Err("Task not found".to_string())
        }
    }

    pub async fn move_to_end(&self, id: Ulid) -> Result<(), String> {
        let task_to_move = {
            let mut tasks = self.tasks.lock().unwrap();
            if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                let task = tasks.remove(pos);
                Some(task)
            } else {
                None
            }
        };

        if let Some(task) = task_to_move {
            let new_pos = {
                let mut tasks = self.tasks.lock().unwrap();
                tasks.push(task);
                tasks.len() as i64 - 1
            };
            crate::database::update_task_position(&self.db, &id, new_pos)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    pub fn get_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.clone()
    }
}

impl Default for TaskStack {
    fn default() -> Self {
        todo!("Implement default for TaskStack")
    }
}

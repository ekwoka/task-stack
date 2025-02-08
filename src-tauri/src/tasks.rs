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
        let tasks = database::get_current_tasks(&db)
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

    pub fn first_active(&self) -> Option<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.iter().find(|t| t.state == TaskState::Active).cloned()
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
        println!("TaskStack: completing task {}", id);

        // First update the database state
        database::update_task_state(&self.db, &id, TaskState::Completed, Some(Utc::now()))
            .await
            .map_err(|e| {
                println!("TaskStack: failed to update database: {}", e);
                e.to_string()
            })?;

        // Then update the task state in memory
        let task = {
            let mut tasks = self.tasks.lock().unwrap();
            let pos = tasks
                .iter()
                .position(|t| t.id == id)
                .ok_or("Task not found")?;
            let mut task = tasks[pos].clone();
            task.state = TaskState::Completed;
            task.completed_at = Some(Utc::now());
            tasks[pos] = task.clone();
            task
        };

        println!("TaskStack: task completed");
        Ok(task)
    }

    pub async fn move_to_end(&self, id: Ulid) -> Result<(), String> {
        // Move task to end and collect positions
        let task_positions = {
            let mut tasks = self.tasks.lock().unwrap();
            let pos = tasks
                .iter()
                .position(|t| t.id == id)
                .ok_or("Task not found")?;
            let task = tasks.remove(pos);
            tasks.push(task);

            // Collect all task IDs and their positions
            tasks
                .iter()
                .enumerate()
                .map(|(i, t)| (t.id, (i + 1) as i64))
                .collect::<Vec<_>>()
        };

        // Update positions for all tasks
        for (task_id, position) in task_positions {
            database::update_task_position(&self.db, &task_id, position)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub async fn get_tasks(&self) -> Vec<Task> {
        database::get_all_tasks(&self.db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|(task, _)| task)
            .collect()
    }

    pub async fn get_current_tasks(&self) -> Vec<Task> {
        database::get_current_tasks(&self.db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|(task, _)| task)
            .collect()
    }
}

impl Default for TaskStack {
    fn default() -> Self {
        todo!("Implement default for TaskStack")
    }
}

use crate::database;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskState {
    Active,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Ulid,
    pub list_id: Ulid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub state: TaskState,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(title: String, list_id: Ulid) -> Self {
        Self {
            id: Ulid::new(),
            list_id,
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
    db: libsql::Database,
    list_id: std::sync::Mutex<Ulid>,
}

impl TaskStack {
    pub fn new(db: libsql::Database, list_id: Ulid) -> Self {
        Self {
            db,
            list_id: std::sync::Mutex::new(list_id),
        }
    }

    pub fn get_list_id(&self) -> Ulid {
        *self.list_id.lock().unwrap()
    }

    pub fn set_list_id(&self, list_id: Ulid) {
        *self.list_id.lock().unwrap() = list_id
    }

    pub async fn push(&self, title: String, description: Option<String>) -> Result<(), String> {
        let task = Task {
            id: Ulid::new(),
            list_id: self.get_list_id(),
            title,
            description,
            created_at: Utc::now(),
            state: TaskState::Active,
            completed_at: None,
        };

        let position = database::get_all_tasks(&self.db, &self.get_list_id())
            .await
            .map_err(|e| e.to_string())?
            .len() as i64;

        database::insert_task(&self.db, &task, position)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn pop(&self) -> Result<Option<Task>, String> {
        let tasks = database::get_all_tasks(&self.db, &self.get_list_id())
            .await
            .map_err(|e| e.to_string())?;

        if let Some((task, _)) = tasks.last() {
            database::delete_task(&self.db, &task.id)
                .await
                .map_err(|e| e.to_string())?;
            Ok(Some(task.clone()))
        } else {
            Ok(None)
        }
    }

    pub async fn first(&self) -> Result<Option<Task>, String> {
        let tasks = database::get_all_tasks(&self.db, &self.get_list_id())
            .await
            .map_err(|e| e.to_string())?;
        Ok(tasks.first().map(|(task, _)| task.clone()))
    }

    pub async fn first_active(&self) -> Result<Option<Task>, String> {
        let tasks = database::get_all_tasks(&self.db, &self.get_list_id())
            .await
            .map_err(|e| e.to_string())?;
        Ok(tasks
            .iter()
            .find(|(task, _)| task.state == TaskState::Active)
            .map(|(task, _)| task.clone()))
    }

    pub async fn size(&self) -> Result<usize, String> {
        let tasks = database::get_all_tasks(&self.db, &self.get_list_id())
            .await
            .map_err(|e| e.to_string())?;
        Ok(tasks.len())
    }

    pub async fn find_task_position(&self, task: &Task) -> Result<usize, String> {
        let tasks = database::get_all_tasks(&self.db, &self.get_list_id())
            .await
            .map_err(|e| e.to_string())?;
        Ok(tasks
            .iter()
            .position(|(t, _)| t.id == task.id)
            .unwrap_or(tasks.len()))
    }

    pub async fn complete_task(&self, id: Ulid) -> Result<Task, String> {
        let tasks = database::get_all_tasks(&self.db, &self.get_list_id())
            .await
            .map_err(|e| e.to_string())?;

        let task = tasks
            .iter()
            .find(|(t, _)| t.id == id)
            .ok_or_else(|| "Task not found".to_string())?
            .0
            .clone();

        let mut updated_task = task.clone();
        updated_task.mark_completed();

        database::update_task_state(&self.db, &id, TaskState::Completed, Some(Utc::now()))
            .await
            .map_err(|e| e.to_string())?;

        Ok(updated_task)
    }

    pub async fn move_to_end(&self, id: Ulid) -> Result<(), String> {
        let tasks = database::get_all_tasks(&self.db, &self.get_list_id())
            .await
            .map_err(|e| e.to_string())?;

        let new_position = tasks.iter().map(|(_, pos)| pos).max().unwrap_or(&0) + 1;

        database::update_task_position(&self.db, &id, new_position)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn get_tasks(&self) -> Result<Vec<Task>, String> {
        let tasks = database::get_all_tasks(&self.db, &self.get_list_id())
            .await
            .map_err(|e| e.to_string())?;
        Ok(tasks.into_iter().map(|(task, _)| task).collect())
    }

    pub async fn get_current_tasks(&self) -> Result<Vec<Task>, String> {
        let tasks = database::get_current_tasks(&self.db, &self.get_list_id())
            .await
            .map_err(|e| e.to_string())?;
        Ok(tasks.into_iter().map(|(task, _)| task).collect())
    }

    pub async fn find_task(&self, id: &Ulid) -> Result<Task, String> {
        let tasks = database::get_all_tasks(&self.db, &self.get_list_id())
            .await
            .map_err(|e| e.to_string())?;

        tasks
            .into_iter()
            .find(|(task, _)| task.id == *id)
            .map(|(task, _)| task)
            .ok_or_else(|| "Task not found".to_string())
    }
}

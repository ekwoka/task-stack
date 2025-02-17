use crate::tasks::{Task, TaskState};
use chrono::{DateTime, Utc};
use libsql::{params, Builder, Database};
use std::path::Path;
use ulid::Ulid;

pub async fn init_database(db_path: &Path) -> Result<Database, libsql::Error> {
    let db = Builder::new_local(db_path).build().await?;

    // Create tables if they don't exist
    let conn = db.connect()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            created_at TEXT NOT NULL,
            state TEXT NOT NULL,
            completed_at TEXT,
            position INTEGER
        )",
        params![],
    )
    .await?;

    Ok(db)
}

pub async fn insert_task(db: &Database, task: &Task, position: i64) -> Result<(), libsql::Error> {
    let conn = db.connect()?;
    conn.execute(
        "INSERT INTO tasks (id, title, description, created_at, state, completed_at, position)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            task.id.to_string(),
            task.title.clone(),
            task.description.clone(),
            task.created_at.to_rfc3339(),
            match task.state {
                TaskState::Active => "active",
                TaskState::Completed => "completed",
            },
            task.completed_at.map(|dt| dt.to_rfc3339()),
            position,
        ],
    )
    .await?;
    Ok(())
}

pub async fn get_all_tasks(db: &Database) -> Result<Vec<(Task, i64)>, libsql::Error> {
    let conn = db.connect()?;
    let mut stmt = conn
        .prepare(
            "SELECT id, title, description, created_at, state, completed_at, position
             FROM tasks
             ORDER BY
                CASE state
                    WHEN 'completed' THEN 0
                    ELSE 1
                END,
                CASE state
                    WHEN 'completed' THEN completed_at
                    ELSE NULL
                END DESC NULLS LAST,
                CASE state
                    WHEN 'active' THEN position
                    ELSE NULL
                END DESC NULLS LAST",
        )
        .await?;

    let mut rows = stmt.query(params![]).await?;
    let mut tasks = Vec::new();

    while let Some(row) = rows.next().await? {
        let id: String = row.get(0)?;
        let title: String = row.get(1)?;
        let description: Option<String> = row.get(2)?;
        let created_at: String = row.get(3)?;
        let state: String = row.get(4)?;
        let completed_at: Option<String> = row.get(5)?;
        let position: i64 = row.get(6)?;

        tasks.push((
            Task {
                id: Ulid::from_string(&id).unwrap(),
                title,
                description,
                created_at: DateTime::parse_from_rfc3339(&created_at)
                    .unwrap()
                    .with_timezone(&Utc),
                state: match state.as_str() {
                    "active" => TaskState::Active,
                    "completed" => TaskState::Completed,
                    _ => TaskState::Active,
                },
                completed_at: completed_at.map(|dt| {
                    DateTime::parse_from_rfc3339(&dt)
                        .unwrap()
                        .with_timezone(&Utc)
                }),
            },
            position,
        ));
    }
    Ok(tasks)
}

pub async fn get_current_tasks(db: &Database) -> Result<Vec<(Task, i64)>, libsql::Error> {
    let conn = db.connect()?;
    let mut stmt = conn
        .prepare(
            "SELECT id, title, description, created_at, state, completed_at, position
         FROM tasks
         WHERE state = 'active' OR (state = 'completed' AND completed_at >= datetime('now', '-12 hours'))
         ORDER BY position DESC",
        )
        .await?;

    let mut rows = stmt.query(params![]).await?;
    let mut tasks = Vec::new();

    while let Some(row) = rows.next().await? {
        let id: String = row.get(0)?;
        let title: String = row.get(1)?;
        let description: Option<String> = row.get(2)?;
        let created_at: String = row.get(3)?;
        let state: String = row.get(4)?;
        let completed_at: Option<String> = row.get(5)?;
        let position: i64 = row.get(6)?;

        tasks.push((
            Task {
                id: Ulid::from_string(&id).unwrap(),
                title,
                description,
                created_at: DateTime::parse_from_rfc3339(&created_at)
                    .unwrap()
                    .with_timezone(&Utc),
                state: match state.as_str() {
                    "active" => TaskState::Active,
                    "completed" => TaskState::Completed,
                    _ => TaskState::Active,
                },
                completed_at: completed_at.map(|dt| {
                    DateTime::parse_from_rfc3339(&dt)
                        .unwrap()
                        .with_timezone(&Utc)
                }),
            },
            position,
        ));
    }
    Ok(tasks)
}

pub async fn update_task_state(
    db: &Database,
    id: &Ulid,
    state: TaskState,
    completed_at: Option<DateTime<Utc>>,
) -> Result<(), libsql::Error> {
    println!("Updating task state in database for ID: {}", id);
    let conn = db.connect()?;
    let state_str = match state {
        TaskState::Active => "active",
        TaskState::Completed => "completed",
    };
    let completed_at_str = completed_at.map(|dt| dt.to_rfc3339());
    println!(
        "Setting state to {} and completed_at to {:?}",
        state_str, completed_at_str
    );
    conn.execute(
        "UPDATE tasks
         SET state = ?, completed_at = ?
         WHERE id = ?",
        params![state_str, completed_at_str, id.to_string(),],
    )
    .await?;
    println!("Database update successful");
    Ok(())
}

pub async fn update_task_position(
    db: &Database,
    id: &Ulid,
    position: i64,
) -> Result<(), libsql::Error> {
    let conn = db.connect()?;
    conn.execute(
        "UPDATE tasks SET position = ? WHERE id = ?",
        params![position, id.to_string()],
    )
    .await?;
    Ok(())
}

pub async fn delete_task(db: &Database, id: &Ulid) -> Result<(), libsql::Error> {
    let conn = db.connect()?;
    conn.execute("DELETE FROM tasks WHERE id = ?", params![id.to_string()])
        .await?;
    Ok(())
}

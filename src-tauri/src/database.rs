use crate::tasks::{Task, TaskState};
use chrono::{DateTime, Utc};
use libsql::{de::from_row, params, Builder, Database};
use std::path::Path;
use ulid::Ulid;

pub async fn init_database(db_path: &Path) -> Result<Database, libsql::Error> {
    let db = Builder::new_local(db_path).build().await?;

    println!("Initialized database at: {:?}", db_path);
    println!("datetime format: {}", Utc::now().to_rfc3339());
    // Create tables if they don't exist
    let conn = db.connect()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            list_id TEXT NOT NULL,
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
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasklists (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        params![],
    )
    .await?;

    Ok(db)
}

pub async fn insert_task(db: &Database, task: &Task, position: i64) -> Result<(), libsql::Error> {
    let conn = db.connect()?;
    conn.execute(
        "INSERT INTO tasks (id, list_id, title, description, created_at, state, completed_at, position)
         VALUES (?, ?, ?, ?, strftime('%FT%R:%f+00:00'), ?, ?, ?)",
        params![
            task.id.to_string(),
            task.list_id.to_string(),
            task.title.clone(),
            task.description.clone(),
            match task.state {
                TaskState::Active => "Active",
                TaskState::Completed => "Completed",
            },
            task.completed_at.map(|dt| dt.to_rfc3339()),
            position,
        ],
    )
    .await?;
    Ok(())
}

pub async fn get_all_tasks(
    db: &Database,
    list_id: &Ulid,
) -> Result<Vec<(Task, i64)>, libsql::Error> {
    let conn = db.connect()?;
    let mut stmt = conn
        .prepare(
            "SELECT id, list_id, title, description, created_at, state, completed_at, position
             FROM tasks
             WHERE list_id = ?
             ORDER BY
                CASE state
                    WHEN 'Completed' THEN 0
                    ELSE 1
                END,
                CASE state
                    WHEN 'Completed' THEN completed_at
                    ELSE NULL
                END DESC NULLS LAST,
                CASE state
                    WHEN 'Active' THEN position
                    ELSE NULL
                END ASC NULLS LAST",
        )
        .await?;

    let mut rows = stmt.query(params![list_id.to_string()]).await?;
    let mut tasks = Vec::new();

    while let Some(row) = rows.next().await? {
        let task: Task = from_row(&row).expect("Row to be valid");
        let position: i64 = row.get(7)?;

        tasks.push((task, position));
    }

    Ok(tasks)
}

pub async fn get_current_tasks(
    db: &Database,
    list_id: &Ulid,
) -> Result<Vec<(Task, i64)>, libsql::Error> {
    let conn = db.connect()?;
    let mut stmt = conn
        .prepare(
            "SELECT id, list_id, title, description, created_at, state, completed_at, position
         FROM tasks
         WHERE list_id = ? AND (state = 'Active' OR (state = 'Completed' AND completed_at >= strftime('%FT%R:%f+00:00', 'now', '-12 hours')))
         ORDER BY position ASC",
        )
        .await?;

    let mut rows = stmt.query(params![list_id.to_string()]).await?;
    let mut tasks = Vec::new();

    while let Some(row) = rows.next().await? {
        let task: Task = from_row(&row).expect("Row to be valid");
        let position: i64 = row.get(7)?;

        tasks.push((task, position));
    }
    Ok(tasks)
}

pub async fn get_active_tasks(
    db: &Database,
    list_id: &Ulid,
) -> Result<Vec<(Task, i64)>, libsql::Error> {
    let conn = db.connect()?;
    let mut stmt = conn
        .prepare(
            "SELECT id, list_id, title, description, created_at, state, completed_at, position
         FROM tasks
         WHERE list_id = ? AND (state = 'Active')
         ORDER BY position ASC",
        )
        .await?;

    let mut rows = stmt.query(params![list_id.to_string()]).await?;
    let mut tasks = Vec::new();

    while let Some(row) = rows.next().await? {
        let task: Task = from_row(&row).expect("Row to be valid");
        let position: i64 = row.get(7)?;

        tasks.push((task, position));
    }
    Ok(tasks)
}

pub async fn get_first_active_task(
    db: &Database,
    list_id: &Ulid,
) -> Result<Option<Task>, libsql::Error> {
    let conn = db.connect()?;
    let mut stmt = conn
        .prepare(
            "SELECT id, list_id, title, description, created_at, state, completed_at, position
         FROM tasks
         WHERE list_id = ? AND (state = 'Active')
         ORDER BY position ASC
         LIMIT 1",
        )
        .await?;

    let mut rows = stmt.query(params![list_id.to_string()]).await?;

    if let Some(row) = rows.next().await? {
        let task: Task = from_row(&row).expect("Row to be valid");
        return Ok(Some(task));
    }
    Ok(None)
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
        TaskState::Active => "Active",
        TaskState::Completed => "Completed",
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

pub async fn get_lists(db: &Database) -> Result<Vec<Ulid>, libsql::Error> {
    let conn = db.connect()?;
    let mut stmt = conn
        .prepare(
            "SELECT id
             FROM tasklists",
        )
        .await?;
    let mut rows = stmt.query(params![]).await?;
    let mut lists = Vec::new();
    while let Some(row) = rows.next().await? {
        let id: String = row.get(0)?;
        lists.push(Ulid::from_string(&id).expect("ID to be valid"));
    }
    Ok(lists)
}

pub async fn create_list(db: &Database, name: &str) -> Result<Ulid, libsql::Error> {
    let conn = db.connect()?;
    let id = Ulid::new();
    conn.execute(
        "INSERT INTO tasklists (id, name, created_at) VALUES (?, ?, strftime('%FT%R:%f+00:00'))",
        params![id.to_string(), name],
    )
    .await?;
    Ok(id)
}

pub async fn get_highest_position(db: &Database, list_id: &Ulid) -> Result<i64, libsql::Error> {
    let conn = db.connect()?;
    let mut stmt = conn
        .prepare("SELECT MAX(position) FROM tasks WHERE list_id = ?")
        .await?;
    let mut rows = stmt.query(params![list_id.to_string()]).await?;
    let mut position = None;
    while let Some(row) = rows.next().await? {
        let max_position: Option<i64> = row.get(0)?;
        position = max_position;
    }
    Ok(position.unwrap_or(0))
}

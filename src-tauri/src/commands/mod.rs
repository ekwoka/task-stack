use crate::tasks::{Task, TaskStack};
use crate::types::{DomUpdate, PageResponse};
use crate::ui::pages::render_index_page;
use tauri::State;
use ulid::Ulid;

#[tauri::command]
pub async fn index(stack: State<'_, TaskStack>) -> Result<PageResponse, String> {
    Ok(PageResponse::new(DomUpdate::from(
        render_index_page(&stack),
        "#app",
        "replace",
    )))
}

#[tauri::command]
pub async fn add_task(
    stack: State<'_, TaskStack>,
    title: String,
    description: Option<String>,
) -> Result<PageResponse, String> {
    let mut task = Task::new(title);
    task.description = description;
    stack.push(task).await;
    Ok(PageResponse::new(DomUpdate::from(
        render_index_page(&stack),
        "#app",
        "replace",
    )))
}

#[tauri::command]
pub async fn complete_task(stack: State<'_, TaskStack>, id: Ulid) -> Result<PageResponse, String> {
    stack.complete_task(id).await?;
    Ok(PageResponse::new(DomUpdate::from(
        render_index_page(&stack),
        "#app",
        "replace",
    )))
}

#[tauri::command]
pub async fn move_task_to_end(
    stack: State<'_, TaskStack>,
    id: Ulid,
) -> Result<PageResponse, String> {
    stack.move_to_end(id).await?;
    Ok(PageResponse::new(DomUpdate::from(
        render_index_page(&stack),
        "#app",
        "replace",
    )))
}

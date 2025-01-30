use crate::tasks::{Task, TaskStack};
use crate::types::{DomUpdate, PageResponse};
use crate::ui::pages::render_index_page;
use tauri::State;

#[tauri::command]
pub fn index(stack: State<TaskStack>) -> Result<PageResponse, String> {
    Ok(PageResponse::new(DomUpdate::from(
        render_index_page(&stack),
        "#app",
        "replace",
    )))
}

#[tauri::command]
pub fn add_task(
    title: String,
    description: Option<String>,
    stack: State<TaskStack>,
) -> Result<PageResponse, String> {
    let mut task = Task::new(title);
    if let Some(desc) = description {
        if !desc.trim().is_empty() {
            task.description = Some(desc);
        }
    }
    stack.push(task);

    // Return updated task list HTML
    Ok(PageResponse::new(DomUpdate::from(
        render_index_page(&stack),
        "#app",
        "replace",
    )))
}

#[tauri::command]
pub fn complete_task(stack: State<TaskStack>, id: ulid::Ulid) -> Result<PageResponse, String> {
    stack.complete_task(id)?;

    Ok(PageResponse::new(DomUpdate::from(
        render_index_page(&stack),
        "#app",
        "replace",
    )))
}

#[tauri::command]
pub fn move_task_to_end(stack: State<TaskStack>, id: ulid::Ulid) -> Result<PageResponse, String> {
    stack.move_to_end(id)?;

    Ok(PageResponse::new(DomUpdate::from(
        render_index_page(&stack),
        "#app",
        "replace",
    )))
}

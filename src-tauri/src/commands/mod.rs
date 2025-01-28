use crate::tasks::{Task, TaskStack};
use crate::types::{DomUpdate, PageResponse};
use crate::ui::pages::render_index_page;
use tauri::State;

#[tauri::command]
pub fn index(stack: State<TaskStack>) -> Result<PageResponse, String> {
    let html = format!("{:#}", render_index_page(&stack));
    Ok(PageResponse {
        updates: vec![DomUpdate {
            target: "#app".to_string(),
            action: "replace".to_string(),
            html,
        }],
        notification: None,
    })
}

#[tauri::command]
pub fn add_task(title: String, stack: State<TaskStack>) -> Result<PageResponse, String> {
    let task = Task::new(title);
    stack.push(task);

    // Return updated task list HTML
    let html = format!("{:#}", render_index_page(&stack));
    Ok(PageResponse {
        updates: vec![DomUpdate {
            target: "#app".to_string(),
            action: "replace".to_string(),
            html,
        }],
        notification: None,
    })
}

#[tauri::command]
pub fn complete_task(stack: State<TaskStack>, id: ulid::Ulid) -> Result<PageResponse, String> {
    stack.remove_task(id)?;

    let html = format!("{:#}", render_index_page(&stack));
    Ok(PageResponse {
        updates: vec![DomUpdate {
            target: "#app".to_string(),
            action: "replace".to_string(),
            html,
        }],
        notification: None,
    })
}

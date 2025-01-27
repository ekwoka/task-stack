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
pub fn complete_task(stack: State<TaskStack>) -> Result<PageResponse, String> {
    if let Some(task) = stack.pop() {
        let html = format!("{:#}", render_index_page(&stack));
        Ok(PageResponse {
            updates: vec![DomUpdate {
                target: "#app".to_string(),
                action: "replace".to_string(),
                html,
            }],
            notification: None,
        })
    } else {
        Err("No tasks to complete".to_string())
    }
}

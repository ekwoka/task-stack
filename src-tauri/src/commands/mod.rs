use crate::tasks::TaskStack;
use crate::types::{DomUpdate, PageResponse};
use crate::ui::pages::{render_index_page, render_list_page};
use tauri::State;
use ulid::Ulid;

#[tauri::command]
pub async fn index(stack: State<'_, TaskStack>) -> Result<PageResponse, String> {
    Ok(PageResponse::new(DomUpdate::from(
        render_index_page(&stack).await,
        "#app",
        "replace",
    )))
}
#[tauri::command]
pub async fn set_list_id(state: State<'_, TaskStack>, list_id: String) -> Result<(), String> {
    let list_id = Ulid::from_string(&list_id).map_err(|e| e.to_string())?;
    state.set_list_id(list_id);
    Ok(())
}

#[tauri::command]
pub async fn get_list_id(state: State<'_, TaskStack>) -> Result<Option<String>, String> {
    Ok(Some(state.get_list_id().to_string()))
}

#[tauri::command]
pub async fn add_task(
    stack: State<'_, TaskStack>,
    title: String,
    description: Option<String>,
) -> Result<PageResponse, String> {
    stack.push(title, description).await?;
    Ok(PageResponse::new(DomUpdate::from(
        render_index_page(&stack).await,
        "#app",
        "replace",
    )))
}

#[tauri::command]
pub async fn complete_task(
    stack: State<'_, TaskStack>,
    id: String,
) -> Result<PageResponse, String> {
    println!("Completing task with ID: {}", id);
    let id = Ulid::from_string(&id).map_err(|e| {
        println!("Failed to parse ID: {}", e);
        e.to_string()
    })?;
    stack.complete_task(id).await.map_err(|e| {
        println!("Failed to complete task: {}", e);
        e.to_string()
    })?;
    println!("Task completed successfully");
    Ok(PageResponse::new(DomUpdate::from(
        render_index_page(&stack).await,
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
        render_index_page(&stack).await,
        "#app",
        "replace",
    )))
}

#[tauri::command]
pub async fn list(stack: State<'_, TaskStack>) -> Result<PageResponse, String> {
    Ok(PageResponse::new(DomUpdate::from(
        render_list_page(&stack).await,
        "#app",
        "replace",
    )))
}

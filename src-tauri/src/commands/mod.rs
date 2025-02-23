use crate::tasks::TaskStack;
use crate::types::{DomUpdate, PageResponse};
use crate::ui::pages;
use html_node::{html, text};
use tauri::State;
use ulid::Ulid;

#[tauri::command]
pub async fn index(stack: State<'_, TaskStack>) -> Result<PageResponse, String> {
    Ok(PageResponse::new(DomUpdate::from(
        pages::index::render(&stack).await,
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
        pages::index::render(&stack).await,
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
        pages::index::render(&stack).await,
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
        pages::index::render(&stack).await,
        "#app",
        "replace",
    )))
}

#[tauri::command]
pub async fn list(stack: State<'_, TaskStack>) -> Result<PageResponse, String> {
    Ok(PageResponse::new(DomUpdate::from(
        pages::list::render(&stack).await,
        "#app",
        "replace",
    )))
}

#[tauri::command]
pub async fn switch_list(
    state: State<'_, TaskStack>,
    list_id: String,
) -> Result<PageResponse, String> {
    if list_id == "new" {
        let new_list_form = html! {
            <form
                class="flex gap-2 w-full max-w-xs"
                data-command="create_list"
                data-trigger="submit"
                data-payload="{ name: $event.target.name.value }"
            >
                <input
                    type="text"
                    name="name"
                    placeholder="List name"
                    class="block flex-1 bg-white border border-gray-300 rounded-lg py-2 px-3 text-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    required=""
                />
                <button
                    type="submit"
                    class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                >
                    { text!("Create") }
                </button>
            </form>
        };
        Ok(PageResponse::new(DomUpdate::from(
            new_list_form,
            "#list-selector",
            "replace",
        )))
    } else {
        let id = list_id.parse::<Ulid>().map_err(|e| e.to_string())?;
        state.set_list_id(id);
        Ok(PageResponse::new(DomUpdate::from(
            pages::index::render(&state).await,
            "#app",
            "replace",
        )))
    }
}

#[tauri::command]
pub async fn create_list(
    state: State<'_, TaskStack>,
    name: String,
) -> Result<PageResponse, String> {
    let id = state.create_new_list(&name).await?;
    state.set_list_id(id);
    Ok(PageResponse::new(DomUpdate::from(
        pages::index::render(&state).await,
        "#app",
        "replace",
    )))
}

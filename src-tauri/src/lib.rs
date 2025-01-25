use html_node::{html, text};

mod routes;
mod tasks;

use routes::*;
use tasks::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!(
        "{:#}",
        html! {
          <div>
            <h1>{ text!("Hello, {}!", name) }</h1>
          </div>
        }
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let task_stack = TaskStack::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(task_stack)
        .invoke_handler(tauri::generate_handler![
            greet,
            index,
            push_task,
            complete_top_task,
            get_top_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

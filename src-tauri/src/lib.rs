pub mod routes;
pub mod tasks;
pub mod types;

// Re-export the task stack for use in main.rs
pub use tasks::TaskStack;
use routes::index;
use tasks::{push_task, complete_top_task};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let task_stack = TaskStack::new();

    tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
        .manage(task_stack)
        .invoke_handler(tauri::generate_handler![
            index,
            push_task,
            complete_top_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod routes;
pub mod tasks;
pub mod types;

// Re-export the task stack for use in main.rs
use routes::{add_task, complete_task, index};
pub use tasks::TaskStack;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let task_stack = TaskStack::new();

    tauri::Builder::default()
        .setup(|_app| Ok(()))
        .manage(task_stack)
        .invoke_handler(tauri::generate_handler![index, add_task, complete_task,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

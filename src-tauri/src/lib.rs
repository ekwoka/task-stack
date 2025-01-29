pub mod commands;
pub mod tasks;
pub mod types;
pub mod ui;

// Re-export the task stack for use in main.rs
pub use tasks::TaskStack;

use commands::{add_task, complete_task, index, move_task_to_end};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let task_stack = TaskStack::new();

    tauri::Builder::default()
        .setup(|_app| Ok(()))
        .manage(task_stack)
        .invoke_handler(tauri::generate_handler![
            index,
            add_task,
            complete_task,
            move_task_to_end
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

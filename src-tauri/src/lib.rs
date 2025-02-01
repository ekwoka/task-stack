pub mod commands;
pub mod database;
pub mod tasks;
pub mod types;
pub mod ui;

// Re-export the task stack for use in main.rs
pub use tasks::TaskStack;

use commands::{add_task, complete_task, index, move_task_to_end};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::block_on(async move {
                let task_stack = TaskStack::new().await;
                handle.manage(task_stack);
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            index,
            add_task,
            complete_task,
            move_task_to_end,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

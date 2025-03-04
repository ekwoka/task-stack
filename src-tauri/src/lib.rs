pub mod commands;
pub mod database;
pub mod tasks;
pub mod types;
pub mod ui;

// Re-export the task stack for use in main.rs
pub use tasks::{Task, TaskStack};

use tauri::{path::BaseDirectory, Manager};

#[cfg(desktop)]
use tauri_plugin_window_state::StateFlags;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().expect("App Data Dir to be Found");
            std::fs::create_dir_all(&app_data_dir).expect("App Data Dir to be Created");

            let handle = app.handle();

            #[cfg(desktop)]
            let _ = handle.plugin(
                tauri_plugin_window_state::Builder::default()
                    .with_state_flags(StateFlags::all() & !StateFlags::VISIBLE)
                    .build(),
            );

            let db_path = app
                .path()
                .resolve("tasks.db", BaseDirectory::AppData)
                .expect("Path to be resolvable");
            tauri::async_runtime::block_on(async move {
                let db = database::init_database(&db_path)
                    .await
                    .expect("DB to be initialized");
                let binding = database::get_lists(&db)
                    .await
                    .expect("DB Should be queryable");
                let list_id = binding.first();
                if let Some(id) = list_id {
                    let task_stack = TaskStack::new(db, *id);
                    handle.manage(task_stack);
                    Ok::<(), Box<dyn std::error::Error>>(())
                } else {
                    let id = database::create_list(&db, "Initial List")
                        .await
                        .expect("List to be created");
                    let task_stack = TaskStack::new(db, id);
                    handle.manage(task_stack);
                    Ok::<(), Box<dyn std::error::Error>>(())
                }
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
            commands::index,
            commands::list,
            commands::lists,
            commands::add_task,
            commands::complete_task,
            commands::move_task_to_end,
            commands::set_list_id,
            commands::get_list_id,
            commands::create_list,
            commands::switch_list,
        ])
        .run(tauri::generate_context!())
        .expect("Task Stack to start correctly");
}

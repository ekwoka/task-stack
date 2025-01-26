use crate::tasks::{Task, TaskStack};
use crate::types::{Notification, PageResponse};
use html_node::{html, text, Node};
use serde::Serialize;
use tauri::State;

/// Renders the index page HTML
#[tauri::command]
pub fn index(stack: State<TaskStack>) -> Result<PageResponse, String> {
    let top_task = stack.first();

    let html = format!(
        "{:#}",
        html! {
            <div>
                <div id="notification" class="fixed top-4 right-4 max-w-sm w-full hidden">
                    <div class="bg-green-100 border-l-4 border-green-500 text-green-700 p-4 rounded shadow-md">
                        <div class="flex">
                            <div class="flex-shrink-0">
                                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                </svg>
                            </div>
                            <div class="ml-3">
                                <p id="notification-text" class="text-sm font-medium"></p>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="min-h-screen bg-gray-50 py-8">
                    <div class="max-w-3xl mx-auto px-4">
                        <header class="text-center mb-12">
                            <h1 class="text-4xl font-bold text-gray-900">{ text!("Task Queue") }</h1>
                            <p class="mt-2 text-gray-600">{ text!("Focus on one task at a time, in the order they were added") }</p>
                        </header>
                        <main class="space-y-8">
                            <div class="bg-white rounded-xl shadow-sm p-6">
                                <form id="task-form" onsubmit="addTask(event)" class="mb-8">
                                    <div class="mb-4">
                                        <label for="title" class="block text-sm font-medium text-gray-700">Task Title</label>
                                        <input
                                            type="text"
                                            name="title"
                                            id="title"
                                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                            required
                                        />
                                    </div>
                                    <button
                                        type="submit"
                                        class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                    >
                                        { text!("Add Task") }
                                    </button>
                                </form>
                                <div id="current-task">
                                    <h2 class="text-xl font-semibold text-gray-900 mb-4">{ text!("Current Task") }</h2>
                                    {
                                        if let Some(task) = top_task {
                                            render_task(&task)
                                        } else {
                                            render_empty_state()
                                        }
                                    }
                                </div>
                            </div>
                        </main>
                    </div>
                </div>
            </div>
        }
    );

    Ok(PageResponse::new(html))
}

#[derive(Serialize)]
pub struct DomUpdate {
    pub html: String,
    pub target: String,
    pub action: String,
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub updates: Vec<DomUpdate>,
    pub notification: Option<Notification>,
}

fn render_task(task: &Task) -> Node {
    html! {
        <div class="bg-gray-50 rounded-lg p-6">
            <h3 class="text-lg font-medium text-gray-900 mb-2">{ text!("{}", task.title) }</h3>
            <button
                onclick="completeTask()"
                class="mt-4 inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
            >
                { text!("Complete Task") }
            </button>
        </div>
    }
}

fn render_empty_state() -> Node {
    html! {
        <div class="text-center py-8">
            <p class="text-gray-500">{ text!("No tasks in the queue") }</p>
        </div>
    }
}

#[tauri::command]
pub fn add_task(title: String, stack: State<TaskStack>) -> Result<TaskResponse, String> {
    stack.push(Task::new(title));

    let mut updates = Vec::new();

    if let Some(top_task) = stack.first() {
        updates.push(DomUpdate {
            html: render_task(&top_task).to_string(),
            target: "#current-task".to_string(),
            action: "replace".to_string(),
        });
    }

    Ok(TaskResponse {
        updates,
        notification: Some(Notification {
            message: "Task added successfully".to_string(),
            notification_type: "success".to_string(),
            duration: Some(3000),
        }),
    })
}

#[tauri::command]
pub fn complete_task(stack: State<TaskStack>) -> Result<TaskResponse, String> {
    stack.pop();

    let mut updates = Vec::new();

    if let Some(top_task) = stack.first() {
        updates.push(DomUpdate {
            html: render_task(&top_task).to_string(),
            target: "#current-task".to_string(),
            action: "replace".to_string(),
        });
    } else {
        updates.push(DomUpdate {
            html: render_empty_state().to_string(),
            target: "#current-task".to_string(),
            action: "replace".to_string(),
        });
    }

    Ok(TaskResponse {
        updates,
        notification: Some(Notification {
            message: "Task completed".to_string(),
            notification_type: "success".to_string(),
            duration: Some(3000),
        }),
    })
}

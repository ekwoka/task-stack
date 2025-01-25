use crate::tasks::{Task, TaskStack};
use html_node::{html, text};
use tauri::State;

/// Renders the index page HTML
#[tauri::command]
pub fn index(stack: State<TaskStack>) -> Result<String, String> {
    let top_task = stack.0.lock().map_err(|e| e.to_string())?.last().cloned();

    Ok(format!(
        "{:#}",
        html! {
            <div class="min-h-screen bg-gray-50 py-8">
                <div class="max-w-3xl mx-auto px-4">
                    <header class="text-center mb-12">
                        <h1 class="text-4xl font-bold text-gray-900">{ text!("Task Stack") }</h1>
                    </header>
                    <main class="space-y-8">
                        <div class="bg-white rounded-xl shadow-sm p-6">
                            {
                                if let Some(task) = top_task {
                                    html! {
                                        <div>
                                            <h2 class="text-xl font-semibold text-gray-900 mb-4">{ text!("Current Task") }</h2>
                                            <div class="bg-gray-50 rounded-lg p-6">
                                                <h3 class="text-lg font-medium text-gray-900 mb-2">{ text!("{}", task.title) }</h3>
                                                {
                                                    if let Some(desc) = task.description {
                                                        html! { <p class="text-gray-600 mb-6">{ text!("{}", desc) }</p> }
                                                    } else {
                                                        html! {}
                                                    }
                                                }
                                                <button
                                                    class="w-full bg-green-600 hover:bg-green-700 text-white font-medium py-2 px-4 rounded-lg transition-colors"
                                                    onclick="window.completeTask()"
                                                >
                                                    { text!("Complete Task") }
                                                </button>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <div class="text-center py-12">
                                            <p class="text-gray-600">{ text!("No tasks in the stack") }</p>
                                        </div>
                                    }
                                }
                            }
                        </div>
                        <div class="bg-white rounded-xl shadow-sm p-6">
                            <h2 class="text-xl font-semibold text-gray-900 mb-4">{ text!("Add New Task") }</h2>
                            <form id="task-form" class="space-y-4" onsubmit="window.addTask(event)">
                                <div>
                                    <input
                                        type="text"
                                        id="task-title"
                                        placeholder="Task title"
                                        required
                                        class="w-full px-4 py-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                                    />
                                </div>
                                <div>
                                    <textarea
                                        id="task-description"
                                        placeholder="Task description (optional)"
                                        class="w-full px-4 py-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 h-32 resize-none"
                                    ></textarea>
                                </div>
                                <button
                                    type="submit"
                                    class="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors"
                                >
                                    { text!("Add Task") }
                                </button>
                            </form>
                        </div>
                    </main>
                </div>
            </div>
        }
    ))
}

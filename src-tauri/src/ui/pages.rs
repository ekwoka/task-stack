use super::components::{render_empty_state, render_notification, render_task};
use crate::tasks::TaskStack;
use html_node::{html, text, Node};

pub fn render_index_page(stack: &TaskStack) -> Node {
    let task = stack.first_active();
    let total_tasks = stack.size();
    let current_pos = task.as_ref().map(|t| stack.find_task_position(t).unwrap_or(0) + 1).unwrap_or(0);

    html! {
        <div class="min-h-screen bg-gray-50 py-8">
            { render_notification() }
            <div class="max-w-3xl mx-auto px-4">
                <header class="text-center mb-12">
                    <h1 class="text-4xl font-bold text-gray-900">{ text!("Task Stack") }</h1>
                    <p class="mt-2 text-gray-600">{ text!("Focus on one task at a time, in the order they were added") }</p>
                </header>
                <main class="space-y-8">
                    <div class="bg-white rounded-xl shadow-sm p-6" style={format!("padding-bottom: {}px;", 3.min(stack.size().saturating_sub(1)) * 2 + 24)}>
                        <form
                            id="task-form"
                            data-command="add_task"
                            data-trigger="submit"
                            class="mb-8"
                        >
                            <div class="mb-4">
                                <label for="title" class="block text-sm font-medium text-gray-700">Task Title</label>
                                <input
                                    type="text"
                                    name="title"
                                    id="title"
                                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                                    required
                                />
                                            </div>
                            <div class="mb-4">
                                <label for="description" class="block text-sm font-medium text-gray-700">{ text!("Description (optional)") }</label>
                                <textarea name="description" id="description" rows="3" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500" placeholder="Add any additional details about the task..."/>
                            </div>
                            <button
                                type="submit"
                                class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 cursor-pointer"
                            >
                                { text!("Add Task") }
                            </button>
                        </form>
                        <div id="task-list" class="space-y-4">
                            {
                                if let Some(task) = stack.first_active() {
                                    render_task(&task, stack)
                                } else {
                                    render_empty_state()
                                }
                            }
                        </div>
                    </div>
                </main>
            </div>
        </div>
    }
}

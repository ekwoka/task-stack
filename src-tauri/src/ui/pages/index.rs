use crate::{
    tasks::TaskStack,
    ui::components::{navigation, notification, task},
};
use html_node::{html, text, Node};

pub async fn render(stack: &TaskStack) -> Node {
    let task = stack.first_active().await.unwrap_or(None);
    let total_tasks = stack.size().await.unwrap_or(0);
    let current_pos = if let Some(ref task) = task {
        stack.find_task_position(task).await.unwrap_or(0) + 1
    } else {
        1
    };

    html! {
        <div class="min-h-screen bg-gray-50 py-8">
            { notification::render() }
            <div class="max-w-3xl mx-auto px-4">
                <header class="text-center mb-12">
                    <h1 class="text-4xl font-bold text-gray-900">{ text!("Task Stack") }</h1>
                    <p class="mt-2 text-gray-600">{ text!("Focus on one task at a time, in the order they were added") }</p>
                </header>
                { navigation::navigation("index", stack).await }
                <main class="flex flex-col gap-12">
                    <div class="bg-white rounded-xl shadow-sm p-6 flex flex-col gap-12" style={format!("padding-bottom: {}px;", 3.min(total_tasks.saturating_sub(1)) * 2 + 24)}>
                        <div id="task-list" class="space-y-4">
                        {
                            if let Some(task) = task {
                                task::card(current_pos, &task, stack).await
                            } else {
                                task::empty()
                            }
                        }
                        </div>
                        <form
                            id="task-form"
                            data-command="add_task"
                            data-trigger="submit"
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
                    </div>
                </main>
            </div>
        </div>
    }
}

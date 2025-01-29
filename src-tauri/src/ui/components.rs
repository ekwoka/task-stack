use crate::tasks::{Task, TaskStack};
use html_node::{html, text, Node};

pub fn render_task(task: &Task, stack: &TaskStack) -> Node {
    let current_pos = stack.find_task_position(task).unwrap_or(0) + 1;
    let total_tasks = stack.size();
    let remaining_tasks = total_tasks - current_pos;
    html! {
      <div class="relative">
          {
              (1..=3.min(remaining_tasks)).rev().map(|i| {
                  let offset = i * 8;
                  let width_adjustment = i * 4;
                  html! {
                      <div class="absolute bg-white rounded-lg border border-gray-200 h-16 shadow-sm"
                        style={format!("bottom: -{offset}px; left: {width_adjustment}px; right: {width_adjustment}px;")}></div>
                  }
              }).collect::<Vec<_>>()
          }
          <div class="bg-white rounded-lg p-6 relative border border-gray-200 shadow-sm">
              <div class="flex justify-between items-start mb-2">
                  <div class="flex flex-col gap-1">
                    <h3 class="text-lg font-medium text-gray-900">{ text!("{}", task.title) }</h3>
                    <span class="text-xs text-gray-400">{ text!("#{}", task.id) }</span>
                  </div>
                  <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-indigo-100 text-indigo-800">
                      { text!("Task {} of {}", current_pos, total_tasks) }
                  </span>
              </div>
              <div class="flex gap-2">
                  <button
                      data-command="complete_task"
                      data-payload={format!("{{\"id\":\"{}\"}}", task.id)}
                      class="mt-4 inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 cursor-pointer"
                  >
                      { text!("Complete Task") }
                  </button>
                  <button
                      data-command="move_task_to_end"
                      data-payload={format!("{{\"id\":\"{}\"}}", task.id)}
                      class="mt-4 inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md shadow-sm text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 cursor-pointer"
                  >
                      { text!("Move to End") }
                  </button>
              </div>
          </div>
      </div>
    }
}

pub fn render_empty_state() -> Node {
    html! {
        <div class="text-center py-12">
            <p class="text-gray-500">{ text!("No tasks in the stack. Add one to get started!") }</p>
        </div>
    }
}

pub fn render_notification() -> Node {
    html! {
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
    }
}

use crate::tasks::{Task, TaskStack, TaskState};
use html_node::{html, text, Node};

pub async fn card(current_pos: usize, task: &Task, stack: &TaskStack, show_stack: bool) -> Node {
    let total_tasks = stack.size().await.unwrap_or(0);
    let remaining_tasks = if task.state == TaskState::Active {
        total_tasks.saturating_sub(current_pos)
    } else {
        0
    };

    html! {
      <div class="relative">
          {
              if show_stack {
                  (1..=3.min(remaining_tasks)).rev().map(|i| {
                      let offset = i * 8;
                      let width_adjustment = i * 4;
                      html! {
                          <div class="absolute bg-white rounded-lg border border-gray-200 h-16 shadow-sm"
                            style={format!("bottom: -{offset}px; left: {width_adjustment}px; right: {width_adjustment}px;")}></div>
                      }
                  }).collect::<Vec<_>>()
              } else {
                  vec![]
              }
          }
          <div class="bg-white rounded-lg p-6 relative border border-gray-200 shadow-sm">
              <div class="flex justify-between items-start mb-2">
                  <div class="flex flex-col gap-1">
                    <div class="flex items-center gap-2">
                        <h3 class="text-lg font-medium text-gray-900">{ text!("{}", task.title) }</h3>
                        {
                            match task.state {
                                TaskState::Active => html! {
                                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                                        { text!("Active") }
                                    </span>
                                },
                                TaskState::Completed => html! {
                                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                                        { text!("Completed") }
                                    </span>
                                }
                            }
                        }
                    </div>
                    <span class="text-xs text-gray-400">{ text!("#{}", task.id) }</span>
                  </div>
                  <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-indigo-100 text-indigo-800">
                      { text!("Task {current_pos} of {total_tasks}") }
                  </span>
              </div>
              {
                  if let Some(description) = &task.description {
                      html! {
                          <p class="mt-2 text-sm text-gray-600">{ text!("{}", description) }</p>
                      }
                  } else {
                      Node::default()
                  }
              }
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

pub fn empty() -> Node {
    html! {
        <div class="text-center py-12">
            <p class="text-gray-500">{ text!("No tasks in the stack. Add one to get started!") }</p>
        </div>
    }
}

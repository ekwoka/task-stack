use crate::tasks::TaskStack;
use html_node::{html, text, Node};

pub async fn navigation(current_view: &str, stack: &TaskStack) -> Node {
    let current_list_id = stack.get_list_id();
    let lists = stack
        .get_lists()
        .await
        .inspect_err(|e| println!("Failed to get lists: {}", e))
        .unwrap_or_default();
    println!("lists: {:?}", lists);

    html! {
        <nav class="mb-8">
            <div class="max-w-3xl mx-auto">
                <div class="flex flex-col space-y-4">
                    <div class="flex justify-center">
                        <select
                            class="block w-full max-w-xs bg-white border border-gray-300 rounded-lg py-2 px-3 text-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                            data-command="switch_list"
                            data-trigger="change"
                        >
                            {
                                lists.into_iter().map(|list| {
                                    html! {
                                        <option value={list.id.to_string()} selected={list.id == current_list_id}>
                                            { text!("{}", list.name) }
                                        </option>
                                    }
                                }).collect::<Vec<_>>()
                            }
                        </select>
                    </div>
                    <div class="flex justify-center space-x-4">
                        <button
                            class={format!("px-4 py-2 rounded-lg {} {}",
                                if current_view == "index" { "bg-blue-500 text-white" } else { "bg-gray-200 text-gray-700" },
                                "w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium hover:bg-blue-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 cursor-pointer"
                            )}
                            data-command="index"
                            data-trigger="click"
                        >
                            { text!("Single Task") }
                        </button>
                        <button
                            class={format!("px-4 py-2 rounded-lg {} {}",
                                if current_view == "list" { "bg-blue-500 text-white" } else { "bg-gray-200 text-gray-700" },
                                "w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium hover:bg-blue-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 cursor-pointer"
                            )}
                            data-command="list"
                            data-trigger="click"
                        >
                            { text!("All Tasks") }
                        </button>
                    </div>
                </div>
            </div>
        </nav>
    }
}

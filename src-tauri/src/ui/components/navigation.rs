use html_node::{html, text, Node};

pub fn buttons(current_view: &str) -> Node {
    html! {
        <div class="flex justify-center space-x-4 mb-8">
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
    }
}

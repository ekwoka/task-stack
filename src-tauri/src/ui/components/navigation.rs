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
        <nav class="flex items-center justify-between max-w-3xl mx-auto px-4">
            <div class="flex items-center space-x-4 text-sm">
                <button
                    class={format!("cursor-pointer text-gray-600 hover:text-gray-900 transition-colors {}",
                        if current_view == "index" { "text-blue-500" } else { "" }
                    )}
                    data-command="index"
                    data-trigger="click"
                >
                    { text!("Single") }
                </button>
                <button
                    class={format!("cursor-pointer text-gray-600 hover:text-gray-900 transition-colors {}",
                        if current_view == "list" { "text-blue-500" } else { "" }
                    )}
                    data-command="list"
                    data-trigger="click"
                >
                    { text!("All") }
                </button>
            </div>
            <div id="list-selector" class="relative flex items-center">
                <select
                    class="appearance-none bg-transparent text-gray-600 text-sm pr-6 focus:outline-none cursor-pointer hover:text-gray-900 transition-colors border-none"
                    data-command="switch_list"
                    data-trigger="change"
                    data-payload="{ listId: $event.target.value }"
                >
                    {
                        lists.into_iter().map(|list| {
                            if list.id == current_list_id {
                                html! {
                                    <option value={list.id.to_string()} selected="">
                                        { text!("{}", list.name) }
                                    </option>
                                }
                            } else {
                                html! {
                                    <option value={list.id.to_string()}>
                                        { text!("{}", list.name) }
                                    </option>
                                }
                            }
                        }).collect::<Vec<_>>()
                    }
                    <option value="new">{ text!("+ New List") }</option>
                </select>
            </div>
        </nav>
    }
}

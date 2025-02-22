use crate::tasks::TaskStack;
use crate::ui::components::{navigation, notification, task};
use html_node::{html, text, Node};

pub async fn render(stack: &TaskStack) -> Node {
    let tasks = stack.get_tasks().await.unwrap_or(vec![]);

    html! {
        <div class="min-h-screen bg-gray-50 py-8">
            { notification::render() }
            <div class="max-w-3xl mx-auto px-4">
                <header class="text-center mb-12">
                    <h1 class="text-4xl font-bold text-gray-900">{ text!("All Tasks") }</h1>
                    <p class="mt-2 text-gray-600">{ text!("View and manage all your tasks") }</p>
                </header>
                { navigation::buttons("list") }
                <main>
                    <div class="bg-white rounded-xl shadow-sm p-6">
                        <div class="space-y-4">
                            {
                                if tasks.is_empty() {
                                    task::empty()
                                } else {
                                    html! {
                                        <div class="space-y-4">
                                            {
                                              let task_nodes: Vec<Node> = futures::future::join_all(tasks.iter()
                                                .enumerate()
                                                  .map(|(i, task)| task::card(i+1, task, stack))).await;
                                              task_nodes
                                            }
                                        </div>
                                    }
                                }
                            }
                        </div>
                    </div>
                </main>
            </div>
        </div>
    }
}

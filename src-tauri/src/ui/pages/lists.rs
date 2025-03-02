use crate::{tasks::TaskStack, ui::components::navigation};
use html_node::{html, text, Node};
use std::collections::HashMap;
use ulid::Ulid;

pub async fn render(stack: &TaskStack) -> Node {
    let lists = stack.get_lists().await.unwrap_or_default();

    // Get task counts for all lists in a single query
    let db = stack.get_db();

    // Create a HashMap to store counts for each list
    let mut active_counts: HashMap<Ulid, usize> = HashMap::new();
    let mut total_counts: HashMap<Ulid, usize> = HashMap::new();

    // Get active task counts for all lists
    if let Ok(conn) = db.connect() {
        // Query for active tasks
        if let Ok(mut stmt) = conn
            .prepare("SELECT list_id, COUNT(*) FROM tasks WHERE state = 'Active' GROUP BY list_id")
            .await
        {
            if let Ok(mut rows) = stmt.query(libsql::params![]).await {
                while let Ok(Some(row)) = rows.next().await {
                    let list_id_str: Result<String, _> = row.get(0);
                    let count: Result<i64, _> = row.get(1);

                    if let (Ok(list_id_str), Ok(count)) = (list_id_str, count) {
                        if let Ok(list_id) = Ulid::from_string(&list_id_str) {
                            active_counts.insert(list_id, count as usize);
                        }
                    }
                }
            }
        }

        // Query for total tasks
        if let Ok(mut stmt) = conn
            .prepare("SELECT list_id, COUNT(*) FROM tasks GROUP BY list_id")
            .await
        {
            if let Ok(mut rows) = stmt.query(libsql::params![]).await {
                while let Ok(Some(row)) = rows.next().await {
                    let list_id_str: Result<String, _> = row.get(0);
                    let count: Result<i64, _> = row.get(1);

                    if let (Ok(list_id_str), Ok(count)) = (list_id_str, count) {
                        if let Ok(list_id) = Ulid::from_string(&list_id_str) {
                            total_counts.insert(list_id, count as usize);
                        }
                    }
                }
            }
        }
    }

    html! {
        <div class="min-h-screen bg-gray-50 py-8">
            <div class="max-w-3xl mx-auto px-4">
                <header class="text-center mb-12">
                    <h1 class="text-4xl font-bold text-gray-900">{ text!("Lists Overview") }</h1>
                    <p class="mt-2 text-gray-600">{ text!("View and manage all your task lists") }</p>
                </header>
                { navigation::navigation("lists", stack).await }
                <main class="mt-8">
                    <div class="bg-white rounded-xl shadow-sm overflow-hidden">
                        <div class="divide-y divide-gray-200">
                            {
                                if lists.is_empty() {
                                    html! {
                                        <div class="p-6 text-center text-gray-500">
                                            { text!("No lists found. Create your first list to get started.") }
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <div>
                                            {
                                                lists.into_iter().map(|list| {
                                                    let list_id = list.id;

                                                    // Get counts from our HashMaps
                                                    let active_count = active_counts.get(&list_id).copied().unwrap_or(0);
                                                    let total_count = total_counts.get(&list_id).copied().unwrap_or(0);

                                                    html! {
                                                        <div class="p-6 hover:bg-gray-50 transition-colors">
                                                            <div class="flex items-center justify-between">
                                                                <div>
                                                                    <h3 class="text-lg font-medium text-gray-900">{ text!("{}", list.name) }</h3>
                                                                    <p class="mt-1 text-sm text-gray-500">
                                                                        { text!("Created: {}", list.created_at.format("%b %d, %Y")) }
                                                                    </p>
                                                                </div>
                                                                <div class="text-right">
                                                                    <p class="text-sm font-medium text-gray-900">
                                                                        { text!("{} active tasks", active_count) }
                                                                    </p>
                                                                    <p class="mt-1 text-sm text-gray-500">
                                                                        { text!("{} total tasks", total_count) }
                                                                    </p>
                                                                </div>
                                                            </div>
                                                            <div class="mt-4">
                                                                <button
                                                                    class="inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded-full shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 cursor-pointer"
                                                                    data-command="switch_list"
                                                                    data-trigger="click"
                                                                    data-payload={{ format!("{{ listId: '{}' }}", list.id) }}
                                                                >
                                                                    { text!("Switch to list") }
                                                                </button>
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()
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

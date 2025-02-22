use html_node::{html, Node};

pub fn render() -> Node {
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

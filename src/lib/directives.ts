import { invoke } from "@tauri-apps/api/core";
import { handlePageResponse, type PageResponse } from "./ui";

type DirectiveHandler = (el: HTMLElement) => void;

const directives = new Map<string, DirectiveHandler>();

// Register the tauri-invoke directive
directives.set("command", (el: HTMLElement) => {
  const { trigger = 'click' } = el.dataset;
  console.log('hooking up directive', el, trigger);
  
  // Always set up the event listener
  el.addEventListener(trigger, commandHandler);
  
  // If trigger is 'now', dispatch the event immediately
  if (trigger === 'now') {
    el.dispatchEvent(new Event(trigger, { cancelable: true }));
  }
});

const commandHandler = async (event: Event) => {
  event.preventDefault();

  const { currentTarget: el } = event;
  if (!el || !(el instanceof HTMLElement)) return;

  const { payload: payloadStr = '{}', command } = el.dataset;
  if (!command) return;

  try {
    const payload = JSON.parse(payloadStr);

    // Handle form submission if the element is a form
    if (el instanceof HTMLFormElement) {
      const formData = new FormData(el);
      const formPayload = Object.fromEntries(formData.entries());
      Object.assign(payload, formPayload);
    }

    const response = await invoke<PageResponse>(command, payload);
    handlePageResponse(response);

    // Dispatch a custom event with the response
    el.dispatchEvent(
      new CustomEvent("tauri:success", {
        detail: { response },
        bubbles: true,
      })
    );

  } catch (error) {
    console.error(`Error invoking ${command}:`, error);
    handlePageResponse({
      updates: [],
      notification: {
        message: error instanceof Error ? error.message : String(error),
        notification_type: "error",
        duration: 5000
      }
    });
    el.dispatchEvent(
      new CustomEvent("tauri:error", {
        detail: { error },
        bubbles: true,
      })
    );
  }
}

// Initialize directives for a specific root element
function initDirectivesForRoot(root: Element | Document) {
  console.log('initializing directives for root', root);
  directives.forEach((handler, name) =>
    root.querySelectorAll(`[data-${name}]`).forEach((el) =>
      handler(el as HTMLElement)));
}

// Initialize directives and set up mutation observer
export function initDirectives() {
  console.log('setting up directive system');
  
  // Initialize existing elements
  initDirectivesForRoot(document);

  // Set up mutation observer for new elements
  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      // Handle added nodes
      mutation.addedNodes.forEach((node) => {
        if (node instanceof Element) {
          initDirectivesForRoot(node);
        }
      });
    });
  });

  // Start observing
  observer.observe(document.body, {
    childList: true,
    subtree: true
  });

  return () => observer.disconnect();
}

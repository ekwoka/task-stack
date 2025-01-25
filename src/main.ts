import './index.css';
import { invoke } from "@tauri-apps/api/core";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let appRoot: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.innerHTML = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

async function loadIndex() {
  if (appRoot) {
    const html = await invoke("index");
    appRoot.innerHTML = html as string;
  }
}

async function addTask(event: Event) {
  event.preventDefault();
  const titleInput = document.querySelector<HTMLInputElement>("#task-title");
  const descInput = document.querySelector<HTMLTextAreaElement>("#task-description");
  
  if (!titleInput) return;
  
  await invoke("push_task", {
    title: titleInput.value,
    description: descInput?.value || null
  });
  
  // Reset form
  titleInput.value = "";
  if (descInput) descInput.value = "";
  
  // Refresh the view
  await loadIndex();
}

async function completeTask() {
  await invoke("complete_top_task");
  await loadIndex();
}

// Make functions available to the window object for HTML event handlers
declare global {
  interface Window {
    addTask: (event: Event) => Promise<void>;
    completeTask: () => Promise<void>;
  }
}

window.addTask = addTask;
window.completeTask = completeTask;

window.addEventListener("DOMContentLoaded", async () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  appRoot = document.querySelector("#app");
  if (!appRoot) {
    console.error("Could not find #app element");
    return;
  }
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
  await loadIndex();
});

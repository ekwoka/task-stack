import './index.css';
import { invoke } from "@tauri-apps/api/core";

interface Notification {
  message: string;
  type: string;
  duration?: number;
}

interface PageResponse {
  html: string;
  notification?: Notification;
}

let appRoot: HTMLElement | null;

function showNotification(notification: Notification) {
  const notificationEl = document.querySelector('#notification');
  const notificationText = document.querySelector('#notification-text');
  
  if (notificationEl && notificationText) {
    // Update text
    (notificationText as HTMLElement).textContent = notification.message;
    
    // Update colors based on type
    const container = notificationEl.querySelector('div');
    if (container) {
      // Remove existing color classes
      container.classList.remove(
        'bg-green-100', 'border-green-500', 'text-green-700',
        'bg-red-100', 'border-red-500', 'text-red-700'
      );
      
      // Add new color classes
      if (notification.type === 'error') {
        container.classList.add('bg-red-100', 'border-red-500', 'text-red-700');
      } else {
        container.classList.add('bg-green-100', 'border-green-500', 'text-green-700');
      }
    }
    
    // Show notification
    notificationEl.classList.remove('hidden');
    notificationEl.classList.add('animate-fade-in');
    
    // Hide after duration
    setTimeout(() => {
      notificationEl.classList.add('animate-fade-out');
      setTimeout(() => {
        notificationEl.classList.add('hidden');
        notificationEl.classList.remove('animate-fade-in', 'animate-fade-out');
      }, 300);
    }, notification.duration || 3000);
  }
}

async function loadIndex() {
  if (appRoot) {
    const response = await invoke<PageResponse>("index");
    appRoot.innerHTML = response.html;
  }
}

async function addTask(event: Event) {
  event.preventDefault();
  const titleInput = document.querySelector<HTMLInputElement>("#task-title");
  const descInput = document.querySelector<HTMLTextAreaElement>("#task-description");
  
  if (!titleInput) return;
  
  try {
    const response = await invoke<PageResponse>("push_task", {
      title: titleInput.value,
      description: descInput?.value || null
    });
    
    // Reset form
    titleInput.value = "";
    if (descInput) descInput.value = "";
    
    // Update page content
    appRoot!.innerHTML = response.html;
    
    // Show notification if present
    if (response.notification) {
      showNotification(response.notification);
    }
  } catch (error) {
    showNotification({
      message: `Error adding task: ${error}`,
      type: 'error',
      duration: 5000
    });
  }
}

async function completeTask() {
  try {
    const response = await invoke<PageResponse>("complete_top_task");
    
    // Update page content
    appRoot!.innerHTML = response.html;
    
    // Show notification if present
    if (response.notification) {
      showNotification(response.notification);
    }
  } catch (error) {
    showNotification({
      message: `Error completing task: ${error}`,
      type: 'error',
      duration: 5000
    });
  }
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
  appRoot = document.querySelector("#app");
  if (!appRoot) {
    console.error("Could not find #app element");
    return;
  }
  await loadIndex();
});

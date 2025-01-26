import "./index.css";
import { invoke } from "@tauri-apps/api/core";

interface Notification {
  message: string;
  notification_type: string;
  duration?: number;
}

interface DomUpdate {
  html: string;
  target: string;
  action: string;
}

interface PageResponse {
  updates: DomUpdate[];
  notification?: Notification;
}

function showNotification(notification: Notification) {
  const notificationEl = document.querySelector("#notification");
  const notificationText = document.querySelector("#notification-text");

  if (notificationEl && notificationText) {
    // Update text
    (notificationText as HTMLElement).textContent = notification.message;

    // Update colors based on type
    const container = notificationEl.querySelector("div");
    if (container) {
      // Remove existing color classes
      container.classList.remove(
        "bg-green-100",
        "border-green-500",
        "text-green-700",
        "bg-red-100",
        "border-red-500",
        "text-red-700",
      );

      // Add new color classes
      if (notification.notification_type === "error") {
        container.classList.add("bg-red-100", "border-red-500", "text-red-700");
      } else {
        container.classList.add(
          "bg-green-100",
          "border-green-500",
          "text-green-700",
        );
      }
    }

    // Show notification
    notificationEl.classList.remove("hidden");
    notificationEl.classList.add("animate-fade-in");

    // Hide after duration
    setTimeout(() => {
      notificationEl.classList.add("animate-fade-out");
      setTimeout(() => {
        notificationEl.classList.add("hidden");
        notificationEl.classList.remove("animate-fade-in", "animate-fade-out");
      }, 300);
    }, notification.duration || 3000);
  }
}

function applyDomUpdates(updates: DomUpdate[]) {
  updates.forEach((update) => {
    const target = document.querySelector(update.target);
    if (!target) {
      console.error(`Target element ${update.target} not found`);
      return;
    }

    switch (update.action) {
      case "replace":
        target.innerHTML = update.html;
        break;
      case "append":
        target.insertAdjacentHTML("beforeend", update.html);
        break;
      case "prepend":
        target.insertAdjacentHTML("afterbegin", update.html);
        break;
      default:
        console.error(`Unknown DOM update action: ${update.action}`);
    }
  });
}

async function loadIndex() {
  const response = await invoke<PageResponse>("index");
  applyDomUpdates(response.updates);
  if (response.notification) {
    showNotification(response.notification);
  }
}

async function addTask(event: Event) {
  event.preventDefault();
  const form = event.target as HTMLFormElement;
  const formData = new FormData(form);
  const title = formData.get("title") as string;

  if (!title) return;

  try {
    const response = await invoke<PageResponse>("add_task", { title });

    // Apply DOM updates
    applyDomUpdates(response.updates);

    // Show notification if present
    if (response.notification) {
      showNotification(response.notification);
    }

    // Reset form
    form.reset();
  } catch (error) {
    showNotification({
      message: error as string,
      notification_type: "error",
      duration: 5000,
    });
  }
}

async function completeTask() {
  try {
    const response = await invoke<PageResponse>("complete_task");

    // Apply DOM updates
    applyDomUpdates(response.updates);

    // Show notification if present
    if (response.notification) {
      showNotification(response.notification);
    }
  } catch (error) {
    showNotification({
      message: error as string,
      notification_type: "error",
      duration: 5000,
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

loadIndex();

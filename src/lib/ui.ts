export interface Notification {
  message: string;
  notification_type: string;
  duration?: number;
}

export interface DomUpdate {
  html: string;
  target: string;
  action: string;
}

export interface PageResponse {
  updates: DomUpdate[];
  notification?: Notification;
}

export function showNotification(notification: Notification) {
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

      // Show notification
      container.classList.remove("hidden");

      // Hide after duration
      setTimeout(() => {
        container.classList.add("hidden");
      }, notification.duration || 3000);
    }
  }
}

export function applyDomUpdates(updates: DomUpdate[]) {
  updates.forEach((update) => {
    const target = document.querySelector(update.target);
    if (!target) return;

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
    }
  });
}

export function handlePageResponse(response: PageResponse) {
  if (response.updates) {
    applyDomUpdates(response.updates);
  }
  if (response.notification) {
    showNotification(response.notification);
  }
}

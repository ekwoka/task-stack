import "./index.css";
import { invoke } from "@tauri-apps/api/core";
import { initDirectives } from "./lib/directives";
import { handlePageResponse, type PageResponse } from "./lib/ui";

async function loadIndex() {
  const response = await invoke<PageResponse>("index");
  handlePageResponse(response);
}

// Initialize directives when the DOM is loaded
document.addEventListener("DOMContentLoaded", () => {
  const cleanup = initDirectives();
  loadIndex();

  // Clean up observer when window unloads
  window.addEventListener("unload", cleanup);
});

import "./index.css";
import { initDirectives } from "./lib/directives";

// Initialize directives when the DOM is loaded
document.addEventListener("DOMContentLoaded", () => {
  const cleanup = initDirectives();

  // Clean up observer when window unloads
  window.addEventListener("unload", cleanup);
});

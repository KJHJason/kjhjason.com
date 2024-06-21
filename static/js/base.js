htmx.config.includeIndicatorStyles = false;

// check if user is using dark mode
if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    const darkTheme = document.createElement("link");
    darkTheme.rel = "stylesheet";
    darkTheme.href = "https://cdn.jsdelivr.net/npm/@sweetalert2/theme-dark@latest/dark.css";
    document.head.appendChild(darkTheme);
}

document.addEventListener("DOMContentLoaded", () => {
    document.getElementById("footer-year").textContent = new Date().getFullYear().toString();
});

const themeToggleButton = document.getElementById("theme-toggle");

themeToggleButton.addEventListener("click", () => {
  const isDark = document.documentElement.classList.toggle("dark");
  localStorage.theme = isDark ? "dark" : "light";
  document.dispatchEvent(
    new CustomEvent("themechange", { detail: { isDark } }),
  );
});

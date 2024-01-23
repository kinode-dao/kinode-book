document.addEventListener("DOMContentLoaded", () => {
  // Renaming the theme buttons based on their IDs
  const lightButton = document.getElementById("light");
  if (lightButton) lightButton.textContent = "Harmony"; // Keeping the name 'Light'

  const rustButton = document.getElementById("rust");
  if (rustButton) rustButton.textContent = "Meadow"; // New name for 'Rust'

  const coalButton = document.getElementById("coal");
  if (coalButton) coalButton.textContent = "Gnucci"; // New name for 'Coal'

  const ayuButton = document.getElementById("ayu");
  if (ayuButton) ayuButton.textContent = "Wynn"; // New name for 'Ayu'

  const navyButton = document.getElementById("navy");
  if (navyButton) navyButton.textContent = "Light"; // New name for 'Navy'

  const sidebar = document.querySelector(".sidebar-scrollbox");
  if (sidebar) {
    const logo = document.createElement("img");
    logo.src = "assets/KINODE_SYMBOL_BLACK_TRANSPARENT.png";
    logo.alt = "Kinode OS Logo";
    logo.style =
      "height: 40px; width: auto; display: block; margin: 10px auto;";

    sidebar.insertBefore(logo, sidebar.firstChild); // This adds the logo to the top of the sidebar
  }
});

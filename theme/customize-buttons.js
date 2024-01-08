document.addEventListener("DOMContentLoaded", () => {
  // Renaming the theme buttons based on their IDs
  const lightButton = document.getElementById("light");
  if (lightButton) lightButton.textContent = "Light"; // Keeping the name 'Light'

  const rustButton = document.getElementById("rust");
  if (rustButton) rustButton.textContent = "Harmony"; // New name for 'Rust'

  const coalButton = document.getElementById("coal");
  if (coalButton) coalButton.textContent = "Gnucci"; // New name for 'Coal'

  const ayuButton = document.getElementById("ayu");
  if (ayuButton) ayuButton.textContent = "Meadow"; // New name for 'Ayu'

  const navyButton = document.getElementById("navy");
  if (navyButton) navyButton.textContent = "Wynn"; // New name for 'Navy'
});

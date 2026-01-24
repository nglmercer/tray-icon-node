import {
  TrayIconBuilder,
  Menu,
  MenuItemBuilder,
  pollTrayEvents,
  pollMenuEvents,
  Icon,
} from "../index.js";

async function main() {
  console.log("Starting Tray Icon Example...");

  // Create a menu
  const menu = new Menu();

  const item1 = new MenuItemBuilder()
    .withText("Say Hello")
    .withId("hello-item")
    .build();

  const quitItem = new MenuItemBuilder()
    .withText("Quit")
    .withId("quit-item")
    .build();

  menu.appendMenuItem(item1);
  menu.appendMenuItem(quitItem);

  // Create a simple 1x1 red icon
  const icon = Icon.fromRgba(Buffer.from([255, 0, 0, 255]), 1, 1);

  // Build the tray icon
  const trayIcon = new TrayIconBuilder()
    .withTooltip("My Awesome Tray App")
    .withTitle("My App")
    .withIcon(icon)
    .withMenu(menu)
    .build();

  console.log("Tray icon created! Check your system tray.");

  // Event loop
  let running = true;
  while (running) {
    // Poll for tray events (clicks, etc.)
    const trayEvent = pollTrayEvents();
    if (trayEvent) {
      console.log("Tray Event:", trayEvent);
    }

    // Poll for menu events (item clicks)
    const menuEvent = pollMenuEvents();
    if (menuEvent) {
      console.log("Menu Event:", menuEvent);
      if (menuEvent.id === "quit-item") {
        console.log("Quitting...");
        running = false;
        process.exit(0);
      }
      if (menuEvent.id === "hello-item") {
        console.log("Hello from Tray!");
      }
    }

    // Wait a bit to avoid maxing out CPU
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
}

main().catch(console.error);

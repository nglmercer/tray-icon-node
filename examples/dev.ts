import {
  TrayIconBuilder,
  Menu,
  MenuItemBuilder,
  CheckMenuItemBuilder,
  SubmenuBuilder,
  PredefinedMenuItem,
  pollTrayEvents,
  pollMenuEvents,
  Icon,
  initialize,
  update,
} from "../index.js";

// Create a simple red icon (32x32)
const iconData = Buffer.alloc(32 * 32 * 4);
for (let i = 0; i < 32 * 32; i++) {
  // Red pixel in RGBA
  iconData[i * 4 + 0] = 255;
  iconData[i * 4 + 1] = 0;
  iconData[i * 4 + 2] = 0;
  iconData[i * 4 + 3] = 255;
}

async function main() {
  console.log("Starting Tray Icon Example...");
  initialize();

  // Create an icon instance
  const icon = Icon.fromRgba(iconData, 32, 32);

  // Build a menu
  const menu = new Menu();

  // Simple item
  const helloItem = new MenuItemBuilder()
    .withText("Say Hello")
    .withId("hello")
    .build();

  // Checkable item
  const checkItem = new CheckMenuItemBuilder()
    .withText("Notifications")
    .withChecked(true)
    .withId("notifications")
    .build();

  // Quit item
  const quitItem = new MenuItemBuilder()
    .withText("Quit")
    .withId("quit")
    .build();
    
    // Submenu
    const subMenuBuilder = new SubmenuBuilder()
    .withText("More Options");
    
    const submenu = subMenuBuilder.build();
    
    const subItem1 = new MenuItemBuilder()
      .withText("Sub Item 1")
      .withId("sub1")
      .build();

    submenu.appendMenuItem(subItem1);
    
    submenu.appendPredefinedMenuItem(PredefinedMenuItem.separator());

    const subItem2 = new CheckMenuItemBuilder()
        .withText("Sub Check")
        .withChecked(true)
        .withId("sub2")
        .build();
    
    submenu.appendCheckMenuItem(subItem2);

  // Assemble menu
  menu.appendMenuItem(helloItem);
  menu.appendPredefinedMenuItem(PredefinedMenuItem.separator());
  menu.appendCheckMenuItem(checkItem);
  menu.appendSubmenu(submenu);
  menu.appendPredefinedMenuItem(PredefinedMenuItem.separator());
  menu.appendMenuItem(quitItem);

  // Create the tray icon
  const tray = new TrayIconBuilder()
    .withTitle("My App")
    .withTooltip("NAPI Tray Icon")
    .withIcon(icon)
    .withMenu(menu)
    .build(); 

    console.log({ tray });

  // Event loop
  // In a real Node app (e.g. Electron or persistent script), you'd have an event loop or similar.
  // Here we simulate one with polling.
  
  // Initial update to ensure everything is settled
  update();

  let running = true;
  while (running) {
    // Process system events (required for Linux/GTK)
    update();

    // Poll tray events
    const trayEvent = pollTrayEvents();
    if (trayEvent) {
      console.log("Tray icon created. Check your system tray!", {tray, trayEvent});
    }

    // Poll menu events
    const menuEvent = pollMenuEvents();
    if (menuEvent) {
      console.log("Menu Event:", menuEvent);
      
      switch (menuEvent.id) {
        case "hello":
          console.log("Hello there!");
          break;
        case "quit":
            console.log("Quitting application...");
            running = false;
            break;
        case "notifications":
            console.log("Toggled notifications (logic not implemented)");
            break;
        case "sub1":
            console.log("Sub Item 1 clicked!");
            break;
        case "sub2":
            console.log("Sub Check clicked!");
            break;
      }
    }

    // Process events at ~60fps to match UI refresh rates and prevent DBus timeouts
    await new Promise((resolve) => setTimeout(resolve, 16));
  }
}

main().catch((err) => {
  console.error("Error running example:", err);
});

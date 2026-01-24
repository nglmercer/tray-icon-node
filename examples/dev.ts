import { 
  TrayIconBuilder, 
  pollTrayEvents, 
  pollMenuEvents, 
  Icon, 
  initialize,
  update 
} from "../index.js";
import { Menu, MenuItemBuilder, SubmenuBuilder, PredefinedMenuItem } from "../index.js";
/**
 * Generates a simple 32x32 red icon as a Buffer.
 * @returns {Buffer}
 */
export function generateIconData() {
  const iconData = Buffer.alloc(32 * 32 * 4);
  for (let i = 0; i < 32 * 32; i++) {
    iconData[i * 4 + 0] = 255; // R
    iconData[i * 4 + 1] = 0;   // G
    iconData[i * 4 + 2] = 0;   // B
    iconData[i * 4 + 3] = 255; // A
  }
  return iconData;
}
export function createTrayMenu() {
  const menu = new Menu();

  const helloItem = new MenuItemBuilder()
    .withText("Say Hello")
    .withId("hello")
    .build();

  const quitItem = new MenuItemBuilder()
    .withText("Exit")
    .withId("quit")
    .build();

  const subMenu = new SubmenuBuilder()
    .withText("More Options")
    .build();

  subMenu.appendMenuItem(
    new MenuItemBuilder().withText("Sub Item 1").withId("sub1").build()
  );

  menu.appendMenuItem(helloItem);
  menu.appendSubmenu(subMenu);
  menu.appendPredefinedMenuItem(PredefinedMenuItem.separator());
  menu.appendMenuItem(quitItem);

  return menu;
}

// Global reference to prevent Garbage Collection
let tray = null;
let isRunning = true;

/**
 * Handles incoming events from the tray and menu.
 */
function handleEvents() {
  const trayEvent = pollTrayEvents();
  if (trayEvent && trayEvent.eventType) {
  //  console.log(trayEvent.eventType);
  }

  const menuEvent = pollMenuEvents();
  if (menuEvent) {
    console.log("Menu Event:", menuEvent.id);
    
    if (menuEvent.id === "hello") {
      console.log("Hello there!");
    }
    
    if (menuEvent.id === "quit") {
      isRunning = false;
    }
  }
}

async function startApp() {
  console.log("Initializing Tray Icon...");
  
  initialize();

  const icon = Icon.fromRgba(generateIconData(), 32, 32);
  const menu = createTrayMenu();

  tray = new TrayIconBuilder()
    .withTitle("My App")
    .withTooltip("Right click for menu")
    .withIcon(icon)
    .withMenu(menu)
    .build();

  console.log("Tray successfully created.");

  // Main Event Loop
  while (isRunning) {
    update();       // Process Windows messages (via Rust)
    handleEvents();  // Process internal event queues
    
    // Small delay to prevent high CPU usage (~30 FPS)
    await new Promise((resolve) => setTimeout(resolve, 32));
  }

  console.log("Shutting down...");
  tray = null;
  process.exit(0);
}

// Non-blocking background task
setInterval(() => console.log("Heartbeat..."), 10000);

startApp().catch(console.error);
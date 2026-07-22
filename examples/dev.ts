import {
  TrayIconBuilder,
  CheckMenuItemBuilder,
  MenuItemBuilder,
  SubmenuBuilder,
  PredefinedMenuItem,
  Menu,
  Icon,
  TrayIcon,
  initialize,
  update,
  pollTrayEvents,
  pollMenuEvents,
} from "../index.js";

// ---------------------------------------------------------------------------
// Icon generation helpers
// ---------------------------------------------------------------------------

function createSolidIcon(r: number, g: number, b: number, size = 32): Icon {
  const data = Buffer.alloc(size * size * 4);
  for (let i = 0; i < size * size; i++) {
    data[i * 4 + 0] = r;
    data[i * 4 + 1] = g;
    data[i * 4 + 2] = b;
    data[i * 4 + 3] = 255;
  }
  return Icon.fromRgba(data, size, size);
}

function createGradientIcon(size = 32): Icon {
  const data = Buffer.alloc(size * size * 4);
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      const i = (y * size + x) * 4;
      data[i + 0] = Math.round((x / size) * 255);
      data[i + 1] = Math.round((y / size) * 255);
      data[i + 2] = 180;
      data[i + 3] = 255;
    }
  }
  return Icon.fromRgba(data, size, size);
}

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

interface AppState {
  notificationsEnabled: boolean;
  turboMode: boolean;
  darkMode: boolean;
  clickCount: number;
  startTime: number;
}

const state: AppState = {
  notificationsEnabled: true,
  turboMode: false,
  darkMode: false,
  clickCount: 0,
  startTime: Date.now(),
};

// ---------------------------------------------------------------------------
// Menu builder
// ---------------------------------------------------------------------------

function buildMenu(): Menu {
  const menu = new Menu();

  /* ---- Section: Actions ---- */

  const helloItem = new MenuItemBuilder()
    .withText("👋 Say Hello")
    .withId("hello")
    .build();
  menu.appendMenuItem(helloItem, "hello");

  const counterItem = new MenuItemBuilder()
    .withText(`Clicks: ${state.clickCount}`)
    .withId("counter")
    .withEnabled(false)
    .build();
  menu.appendMenuItem(counterItem, "counter");

  menu.appendPredefinedMenuItem(PredefinedMenuItem.separator());

  /* ---- Section: Toggle settings ---- */

  const notifItem = new CheckMenuItemBuilder()
    .withText("🔔 Notifications")
    .withId("toggle_notifications")
    .withChecked(state.notificationsEnabled)
    .build();
  menu.appendCheckMenuItem(notifItem, "toggle_notifications");

  const darkItem = new CheckMenuItemBuilder()
    .withText("🌙 Dark Mode")
    .withId("toggle_darkmode")
    .withChecked(state.darkMode)
    .build();
  menu.appendCheckMenuItem(darkItem, "toggle_darkmode");

  menu.appendPredefinedMenuItem(PredefinedMenuItem.separator());

  /* ---- Section: Submenu (Advanced) ---- */

  const advancedMenu = new SubmenuBuilder()
    .withText("⚙️ Advanced")
    .build();

  const turboItem = new CheckMenuItemBuilder()
    .withText("🚀 Turbo Mode")
    .withId("toggle_turbo")
    .withChecked(state.turboMode)
    .build();
  advancedMenu.appendCheckMenuItem(turboItem, "toggle_turbo");

  const subItem1 = new MenuItemBuilder()
    .withText("Sub Action A")
    .withId("sub_action_a")
    .build();
  advancedMenu.appendMenuItem(subItem1, "sub_action_a");

  const subItem2 = new MenuItemBuilder()
    .withText("Sub Action B")
    .withId("sub_action_b")
    .build();
  advancedMenu.appendMenuItem(subItem2, "sub_action_b");

  /* ---- Nested submenu (Theme selector) ---- */

  const themeMenu = new SubmenuBuilder()
    .withText("🎨 Theme")
    .build();

  for (const [label, id] of [
    ["Red", "theme_red"],
    ["Green", "theme_green"],
    ["Blue", "theme_blue"],
    ["Gradient", "theme_gradient"],
  ] as const) {
    const themeItem = new MenuItemBuilder()
      .withText(label)
      .withId(id)
      .build();
    themeMenu.appendMenuItem(themeItem, id);
  }

  advancedMenu.appendSubmenu(themeMenu, "theme_menu");
  menu.appendSubmenu(advancedMenu, "advanced");

  menu.appendPredefinedMenuItem(PredefinedMenuItem.separator());

  /* ---- Section: Info ---- */

  const uptimeItem = new MenuItemBuilder()
    .withText("Uptime: 0s")
    .withId("uptime")
    .withEnabled(false)
    .build();
  menu.appendMenuItem(uptimeItem, "uptime");

  const aboutItem = new MenuItemBuilder()
    .withText("ℹ️ About")
    .withId("about")
    .build();
  menu.appendMenuItem(aboutItem, "about");

  menu.appendPredefinedMenuItem(PredefinedMenuItem.separator());

  /* ---- Section: Quit ---- */

  const quitItem = new MenuItemBuilder()
    .withText("✕ Quit")
    .withId("quit")
    .build();
  menu.appendMenuItem(quitItem, "quit");

  return menu;
}

// ---------------------------------------------------------------------------
// Dynamic UI updates
// ---------------------------------------------------------------------------

function updateMenuText(menu: Menu): void {
  menu.setText("counter", `Clicks: ${state.clickCount}`);

  const elapsed = Math.floor((Date.now() - state.startTime) / 1000);
  const minutes = Math.floor(elapsed / 60);
  const seconds = elapsed % 60;
  menu.setText("uptime", `Uptime: ${minutes}m ${seconds}s`);

  menu.setText(
    "toggle_notifications",
    `${state.notificationsEnabled ? "🔔" : "🔕"} Notifications`
  );
  menu.setText("toggle_darkmode", `${state.darkMode ? "🌙" : "☀️"} Dark Mode`);
  menu.setText("toggle_turbo", `${state.turboMode ? "🚀" : "🐢"} Turbo Mode`);
}

function updateTooltip(tr: TrayIcon): void {
  const parts: string[] = [`tray-icon-node [${currentIcon}]`];
  if (state.notificationsEnabled) parts.push("🔔");
  if (state.turboMode) parts.push("🚀");
  if (state.darkMode) parts.push("🌙");
  parts.push(`| ${state.clickCount} clicks`);
  tr.setTooltip(parts.join(" "));
}

// ---------------------------------------------------------------------------
// Event handling
// ---------------------------------------------------------------------------

type ThemeName = "red" | "green" | "blue" | "gradient";

let currentIcon: ThemeName = "gradient";
let tray: TrayIcon | null = null;
let menu: Menu | null = null;

const icons: Record<ThemeName, Icon> = {
  red: createSolidIcon(220, 60, 60),
  green: createSolidIcon(60, 200, 100),
  blue: createSolidIcon(60, 120, 220),
  gradient: createGradientIcon(),
};

function changeIcon(theme: ThemeName): void {
  currentIcon = theme;
  if (tray) {
    tray.setIcon(icons[theme]);
  }
  menu?.setText("advanced", `⚙️ Active: ${theme}`);
}

function handleMenuEvent(menu: Menu): void {
  const event = pollMenuEvents();
  if (!event) return;

  switch (event.id) {
    case "hello":
      state.clickCount++;
      console.log("Hello from tray-icon-node! 👋");
      break;

    case "toggle_notifications":
      state.notificationsEnabled = menu.toggleCheck("toggle_notifications");
      console.log(`Notifications: ${state.notificationsEnabled ? "ON" : "OFF"}`);
      break;

    case "toggle_darkmode":
      state.darkMode = menu.toggleCheck("toggle_darkmode");
      console.log(`Dark mode: ${state.darkMode ? "ON" : "OFF"}`);
      break;

    case "toggle_turbo":
      state.turboMode = menu.toggleCheck("toggle_turbo");
      console.log(`Turbo mode: ${state.turboMode ? "ON" : "OFF"}`);
      break;

    case "sub_action_a":
      console.log("Sub Action A triggered");
      break;

    case "sub_action_b":
      console.log("Sub Action B triggered");
      break;

    case "theme_red":
      changeIcon("red");
      console.log("Theme: Red");
      break;

    case "theme_green":
      changeIcon("green");
      console.log("Theme: Green");
      break;

    case "theme_blue":
      changeIcon("blue");
      console.log("Theme: Blue");
      break;

    case "theme_gradient":
      changeIcon("gradient");
      console.log("Theme: Gradient");
      break;

    case "about":
      const uptime = Math.floor((Date.now() - state.startTime) / 1000);
      console.log("=== tray-icon-node ===");
      console.log(`Version: 0.1.1`);
      console.log(`Uptime: ${uptime}s`);
      console.log(`Clicks: ${state.clickCount}`);
      console.log(`Notifications: ${state.notificationsEnabled}`);
      console.log(`Dark mode: ${state.darkMode}`);
      console.log(`Turbo mode: ${state.turboMode}`);
      break;

    case "quit":
      shutdown();
      break;

    default:
      break;
  }
}

function handleTrayEvent(): void {
  const event = pollTrayEvents();
  if (!event) return;

  switch (event.eventType) {
    case "click":
      state.clickCount++;
      console.log(`Tray clicked at (${event.x}, ${event.y}) — button: ${event.button}`);
      break;
    case "double-click":
      state.clickCount += 2;
      console.log("Tray double-clicked");
      break;
    case "enter":
      console.log("Mouse entered tray icon");
      break;
    case "leave":
      console.log("Mouse left tray icon");
      break;
    case "move":
      break;
    default:
      break;
  }
}

// ---------------------------------------------------------------------------
// Lifecycle
// ---------------------------------------------------------------------------

let isRunning = true;

function shutdown(): void {
  console.log("Shutting down...");
  isRunning = false;
}

function setupSignalHandlers(): void {
  const signals: NodeJS.Signals[] = ["SIGINT", "SIGTERM"];
  for (const sig of signals) {
    process.on(sig, () => {
      console.log(`\nReceived ${sig}`);
      shutdown();
    });
  }
}

async function main(): Promise<void> {
  console.log("╔══════════════════════════════════╗");
  console.log("║  tray-icon-node example          ║");
  console.log("║  Right-click the tray icon       ║");
  console.log("╚══════════════════════════════════╝");

  initialize();

  menu = buildMenu();

  tray = new TrayIconBuilder()
    .withIcon(icons.gradient)
    .withTitle("Demo")
    .withTooltip("tray-icon-node — right-click for menu")
    .withMenu(menu)
    .build();

  setupSignalHandlers();

  console.log("Tray icon active. Press Ctrl+C to quit.\n");

  let tick = 0;
  while (isRunning) {
    update();
    handleTrayEvent();
    handleMenuEvent(menu);

    if (tick % 30 === 0) {
      updateMenuText(menu);
      updateTooltip(tray);
    }

    tick++;
    await new Promise((resolve) => setTimeout(resolve, 32));
  }

  tray = null;
  menu = null;
  process.exit(0);
}

main().catch((err) => {
  console.error("Fatal error:", err);
  process.exit(1);
});

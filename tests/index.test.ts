import { expect, test, describe, beforeEach } from "bun:test";
import {
  Icon,
  Menu,
  MenuItemBuilder,
  CheckMenuItemBuilder,
  SubmenuBuilder,
  IconMenuItemBuilder,
  PredefinedMenuItem,
  TrayIconBuilder,
  initialize,
  update,
  pollTrayEvents,
  pollMenuEvents,
} from "../index.js";

let nativeAvailable = false;

try {
  initialize();
  nativeAvailable = true;
} catch {
  nativeAvailable = false;
}

const describeIfNative = nativeAvailable ? describe : describe.skip;

describe("NAPI Module Tests", () => {
  test("initialize runs without error", () => {
    expect(() => initialize()).not.toThrow();
  });

  test("update runs without error", () => {
    expect(() => update()).not.toThrow();
  });

  test("pollTrayEvents returns null when no events", () => {
    const event = pollTrayEvents();
    expect(event === null || typeof event === "object").toBe(true);
  });

  test("pollMenuEvents returns null when no events", () => {
    const event = pollMenuEvents();
    expect(event === null || typeof event === "object").toBe(true);
  });
});

describeIfNative("Icon", () => {
  test("fromRgba creates an icon from RGBA buffer", () => {
    const size = 16 * 16 * 4;
    const rgba = Buffer.alloc(size, 128);
    const icon = Icon.fromRgba(rgba, 16, 16);
    expect(icon).toBeDefined();
  });

  test("fromRgba rejects invalid buffer size", () => {
    const rgba = Buffer.alloc(10);
    expect(() => Icon.fromRgba(rgba, 16, 16)).toThrow();
  });

  test("fromPath rejects non-existent file", () => {
    expect(() => Icon.fromPath("/nonexistent/path/to/image.png")).toThrow();
  });
});

describeIfNative("Menu Builders", () => {
  test("MenuItemBuilder builds with text", () => {
    const item = new MenuItemBuilder()
      .withText("Hello")
      .withEnabled(true)
      .withId("hello")
      .build();
    expect(item).toBeDefined();
  });

  test("MenuItemBuilder builds without id", () => {
    const item = new MenuItemBuilder()
      .withText("No ID")
      .build();
    expect(item).toBeDefined();
  });

  test("MenuItemBuilder setText works", () => {
    const item = new MenuItemBuilder()
      .withText("Original")
      .build();
    item.setText("Updated");
    expect(item).toBeDefined();
  });

  test("MenuItemBuilder setEnabled works", () => {
    const item = new MenuItemBuilder()
      .withText("Test")
      .withEnabled(true)
      .build();
    item.setEnabled(false);
    expect(item).toBeDefined();
  });

  test("CheckMenuItemBuilder builds with all options", () => {
    const item = new CheckMenuItemBuilder()
      .withText("Check me")
      .withEnabled(true)
      .withChecked(true)
      .withId("check1")
      .build();
    expect(item).toBeDefined();
    expect(item.isChecked()).toBe(true);
  });

  test("CheckMenuItemBuilder toggle check state", () => {
    const item = new CheckMenuItemBuilder()
      .withText("Toggle")
      .withChecked(false)
      .build();
    expect(item.isChecked()).toBe(false);
    item.setChecked(true);
    expect(item.isChecked()).toBe(true);
  });

  test("SubmenuBuilder builds with text and enabled", () => {
    const submenu = new SubmenuBuilder()
      .withText("Submenu")
      .withEnabled(true)
      .build();
    expect(submenu).toBeDefined();
  });

  test("IconMenuItemBuilder builds with icon", () => {
    const rgba = Buffer.alloc(16 * 16 * 4, 255);
    const icon = Icon.fromRgba(rgba, 16, 16);
    const item = new IconMenuItemBuilder()
      .withText("Icon Item")
      .withIcon(icon)
      .withId("icon-item")
      .build();
    expect(item).toBeDefined();
  });

  test("IconMenuItemBuilder requires icon", () => {
    expect(() =>
      new IconMenuItemBuilder()
        .withText("No Icon")
        .build()
    ).toThrow();
  });
});

describeIfNative("Menu Operations", () => {
  let menu: Menu;

  beforeEach(() => {
    menu = new Menu();
  });

  test("appendMenuItem adds item to menu", () => {
    const item = new MenuItemBuilder().withText("Test").withId("test").build();
    expect(() => menu.appendMenuItem(item)).not.toThrow();
  });

  test("appendCheckMenuItem adds check item to menu", () => {
    const item = new CheckMenuItemBuilder()
      .withText("Check")
      .withId("check")
      .withChecked(true)
      .build();
    expect(() => menu.appendCheckMenuItem(item, "check")).not.toThrow();
  });

  test("isChecked returns correct state", () => {
    const item = new CheckMenuItemBuilder()
      .withText("Check")
      .withId("check-1")
      .withChecked(true)
      .build();
    menu.appendCheckMenuItem(item, "check-1");
    expect(menu.isChecked("check-1")).toBe(true);
  });

  test("isChecked returns false for unknown id", () => {
    expect(menu.isChecked("nonexistent")).toBe(false);
  });

  test("toggleCheck flips state", () => {
    const item = new CheckMenuItemBuilder()
      .withText("Toggle")
      .withId("toggle-1")
      .withChecked(false)
      .build();
    menu.appendCheckMenuItem(item, "toggle-1");
    const newState = menu.toggleCheck("toggle-1");
    expect(newState).toBe(true);
    expect(menu.isChecked("toggle-1")).toBe(true);
  });

  test("toggleCheck returns false for unknown id", () => {
    expect(menu.toggleCheck("nonexistent")).toBe(false);
  });

  test("setText updates menu item text", () => {
    const item = new MenuItemBuilder()
      .withText("Original")
      .withId("text-item")
      .build();
    menu.appendMenuItem(item, "text-item");
    expect(() => menu.setText("text-item", "Updated")).not.toThrow();
  });

  test("setText on check item works", () => {
    const item = new CheckMenuItemBuilder()
      .withText("Original")
      .withId("text-check")
      .build();
    menu.appendCheckMenuItem(item, "text-check");
    expect(() => menu.setText("text-check", "Updated")).not.toThrow();
  });

  test("appendSubmenu adds submenu to menu", () => {
    const submenu = new SubmenuBuilder().withText("Sub").build();
    expect(() => menu.appendSubmenu(submenu)).not.toThrow();
  });

  test("appendPredefinedMenuItem adds separator", () => {
    const separator = PredefinedMenuItem.separator();
    expect(() => menu.appendPredefinedMenuItem(separator)).not.toThrow();
  });
});

describeIfNative("Submenu Operations", () => {
  test("appendMenuItem to submenu works", () => {
    const submenu = new SubmenuBuilder().withText("Sub").build();
    const item = new MenuItemBuilder().withText("Item").build();
    expect(() => submenu.appendMenuItem(item)).not.toThrow();
  });

  test("appendCheckMenuItem to submenu works", () => {
    const submenu = new SubmenuBuilder().withText("Sub").build();
    const item = new CheckMenuItemBuilder().withText("Check").build();
    expect(() => submenu.appendCheckMenuItem(item)).not.toThrow();
  });

  test("nested submenu works", () => {
    const parent = new SubmenuBuilder().withText("Parent").build();
    const child = new SubmenuBuilder().withText("Child").build();
    expect(() => parent.appendSubmenu(child)).not.toThrow();
  });
});

describeIfNative("TrayIconBuilder", () => {
  test("builds with minimal config", () => {
    expect(() => new TrayIconBuilder().build()).not.toThrow();
  });

  test("builds with icon", () => {
    const rgba = Buffer.alloc(16 * 16 * 4, 255);
    const icon = Icon.fromRgba(rgba, 16, 16);
    const tray = new TrayIconBuilder()
      .withIcon(icon)
      .withTooltip("Test")
      .withTitle("Test App")
      .build();
    expect(tray).toBeDefined();
  });

  test("builds with menu", () => {
    const menu = new Menu();
    const item = new MenuItemBuilder().withText("Item").build();
    menu.appendMenuItem(item);

    const rgba = Buffer.alloc(16 * 16 * 4, 255);
    const icon = Icon.fromRgba(rgba, 16, 16);
    const tray = new TrayIconBuilder()
      .withIcon(icon)
      .withMenu(menu)
      .build();
    expect(tray).toBeDefined();
  });

  test("setIcon changes the tray icon", () => {
    const rgba = Buffer.alloc(16 * 16 * 4, 255);
    const icon = Icon.fromRgba(rgba, 16, 16);
    const tray = new TrayIconBuilder().withIcon(icon).build();

    const newRgba = Buffer.alloc(16 * 16 * 4, 0);
    const newIcon = Icon.fromRgba(newRgba, 16, 16);
    expect(() => tray.setIcon(newIcon)).not.toThrow();
  });

  test("setTooltip changes the tooltip", () => {
    const tray = new TrayIconBuilder().build();
    expect(() => tray.setTooltip("New tooltip")).not.toThrow();
  });

  test("setTitle changes the title", () => {
    const tray = new TrayIconBuilder().build();
    expect(() => tray.setTitle("New title")).not.toThrow();
  });

  test("setVisible toggles visibility", () => {
    const tray = new TrayIconBuilder().build();
    expect(() => tray.setVisible(false)).not.toThrow();
    expect(() => tray.setVisible(true)).not.toThrow();
  });
});

describeIfNative("PredefinedMenuItem", () => {
  test("separator creates a separator", () => {
    const sep = PredefinedMenuItem.separator();
    expect(sep).toBeDefined();
  });
});

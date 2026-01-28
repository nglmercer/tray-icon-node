# tray-icon-node

A cross-platform system tray icon library for Node.js with native Rust bindings.

## Features

- **System Tray Icon** - Create and manage system tray icons on Windows, macOS, and Linux
- **Context Menus** - Build rich context menus with items, checkboxes, submenus, and separators
- **Icon Support** - Load icons from file paths or create from RGBA buffers
- **Event Handling** - Handle tray click events and menu interactions
- **Cross-platform** - Works on Windows, macOS, and Linux (GTK-based)
- **Native Performance** - Rust-based native modules using NAPI-RS
- **TypeScript** - Full TypeScript support with type definitions

## Prerequisites

- Node.js >= 18.0.0
- Bun >= 1.0.0 (for development)
- Rust toolchain (stable)
- Linux: GTK development libraries (`libgtk-3-dev` on Debian/Ubuntu)

## Installation

```bash
bun install tray-icon-node
```

Or with npm:
```bash
npm install tray-icon-node
```

## Quick Start

```typescript
import { 
  TrayIconBuilder, 
  Menu, 
  MenuItemBuilder,
  Icon,
  initialize,
  update,
  pollTrayEvents,
  pollMenuEvents 
} from 'tray-icon-node';

// Initialize platform-specific requirements
initialize();

// Create a menu
const menu = new Menu();
menu.appendMenuItem(
  new MenuItemBuilder()
    .withText("Hello")
    .withId("hello")
    .build()
);

// Create tray icon
const tray = new TrayIconBuilder()
  .withIcon(Icon.fromPath("./icon.png"))
  .withTooltip("My App")
  .withMenu(menu)
  .build();

// Event loop
setInterval(() => {
  update();
  
  const trayEvent = pollTrayEvents();
  if (trayEvent) {
    console.log('Tray clicked:', trayEvent);
  }
  
  const menuEvent = pollMenuEvents();
  if (menuEvent) {
    console.log('Menu clicked:', menuEvent.id);
  }
}, 16);
```

## Building from Source

### Release build
```bash
bun run build
```

### Debug build
```bash
bun run build:debug
```

## Development

Run the example development script:
```bash
bun run dev
```

## Testing

```bash
bun test
```

## API Reference

### Classes

#### `TrayIconBuilder`
Builder for creating system tray icons.

```typescript
const tray = new TrayIconBuilder()
  .withIcon(icon: Icon)
  .withTooltip(tooltip: string)
  .withTitle(title: string)
  .withMenu(menu: Menu)
  .build();
```

#### `TrayIcon`
Represents a system tray icon instance.

```typescript
tray.setIcon(icon?: Icon | null): void
tray.setTooltip(tooltip?: string | null): void
tray.setTitle(title?: string | null): void
tray.setVisible(visible: boolean): void
```

#### `Menu`
Context menu for tray icons.

```typescript
const menu = new Menu();
menu.appendMenuItem(item: MenuItem, id?: string): void
menu.appendCheckMenuItem(item: CheckMenuItem, id: string): void
menu.appendIconMenuItem(item: IconMenuItem, id: string): void
menu.appendSubmenu(item: Submenu, id?: string): void
menu.appendPredefinedMenuItem(item: PredefinedMenuItem): void
menu.isChecked(id: string): boolean
menu.toggleCheck(id: string): boolean
menu.setText(id: string, text: string): void
```

#### `MenuItemBuilder` / `MenuItem`
Standard menu items.

```typescript
const item = new MenuItemBuilder()
  .withText(text: string)
  .withEnabled(enabled: boolean)
  .withId(id: string)
  .build();

item.setText(text: string): void
item.setEnabled(enabled: boolean): void
```

#### `CheckMenuItemBuilder` / `CheckMenuItem`
Checkbox menu items.

```typescript
const item = new CheckMenuItemBuilder()
  .withText(text: string)
  .withEnabled(enabled: boolean)
  .withChecked(checked: boolean)
  .withId(id: string)
  .build();

item.isChecked(): boolean
item.setChecked(checked: boolean): void
```

#### `SubmenuBuilder` / `Submenu`
Nested submenus.

```typescript
const submenu = new SubmenuBuilder()
  .withText(text: string)
  .withEnabled(enabled: boolean)
  .build();

submenu.appendMenuItem(item: MenuItem): void
submenu.appendSubmenu(item: Submenu): void
submenu.appendCheckMenuItem(item: CheckMenuItem): void
submenu.appendIconMenuItem(item: IconMenuItem): void
submenu.appendPredefinedMenuItem(item: PredefinedMenuItem): void
```

#### `Icon`
Icon creation utility.

```typescript
const iconFromFile = Icon.fromPath(path: string);
const iconFromBuffer = Icon.fromRgba(rgba: Buffer, width: number, height: number);
```

#### `PredefinedMenuItem`
Built-in menu items like separators.

```typescript
const separator = PredefinedMenuItem.separator();
```

### Functions

#### `initialize()`
Initializes platform-specific requirements (GTK on Linux). Must be called before creating tray icons.

#### `update()`
Processes pending platform events. Should be called regularly in your event loop.

#### `pollTrayEvents()`
Returns pending tray events or `null` if none.

```typescript
interface TrayIconEvent {
  eventType: string;
  id: string;
  x: number;
  y: number;
  iconRect: Rect;
  button: MouseButton;
  buttonState: MouseButtonState;
}
```

#### `pollMenuEvents()`
Returns pending menu click events or `null` if none.

```typescript
interface MenuEvent {
  id: string;
}
```

### Enums

```typescript
enum MouseButton {
  Left = 0,
  Right = 1,
  Middle = 2
}

enum MouseButtonState {
  Up = 0,
  Down = 1
}
```

## Project Structure

- `src/` - Rust source code
  - `lib.rs` - Main library entry point
  - `tray.rs` - Tray icon implementation
  - `menu.rs` - Menu system implementation
  - `icon.rs` - Icon handling
  - `common.rs` - Shared types and utilities
- `examples/` - Usage examples
- `tests/` - Test suite

## Supported Platforms

| Platform | Architecture | Status |
|----------|-------------|--------|
| Windows | x86_64 | ✅ Supported |
| Windows | i686 | ✅ Supported |
| macOS | x86_64 | ✅ Supported |
| macOS | aarch64 (Apple Silicon) | ✅ Supported |
| Linux | x86_64 | ✅ Supported |
| Linux | aarch64 | ✅ Supported |

## License

MIT

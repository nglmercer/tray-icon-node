use napi::bindgen_prelude::*;
use napi_derive::napi;

pub mod common;
pub mod icon;
pub mod menu;
pub mod tray;

pub use common::*;
pub use icon::*;
pub use menu::*;
pub use tray::*;

/// Initializes the native tray subsystem.
///
/// On Linux, this initializes GTK. On macOS and Windows, this is a no-op
/// as those platforms handle initialization automatically.
///
/// # Errors
///
/// Returns an error if GTK initialization fails (Linux only).
#[napi]
pub fn initialize() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        if gtk::init().is_err() {
            return Err(Error::from_reason("Failed to initialize GTK"));
        }
    }
    Ok(())
}

/// Processes pending UI events for the tray subsystem.
///
/// On Linux, this iterates pending GTK events.
/// On Windows, this processes the thread's message queue via `PeekMessageW`.
/// On macOS, this is a no-op as `NSApplication` handles its own event loop.
///
/// Call this in your event loop to ensure tray interactions are processed.
#[napi]
pub fn update() {
    #[cfg(target_os = "linux")]
    {
        while gtk::events_pending() {
            gtk::main_iteration_do(false);
        }
    }

    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE,
        };
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            while PeekMessageW(&mut msg, 0, 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // No-op: macOS uses NSApplication's built-in run loop.
    }
}

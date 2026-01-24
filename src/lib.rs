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
            // Procesa todos los mensajes pendientes en la cola de Windows
            while PeekMessageW(&mut msg, 0, 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

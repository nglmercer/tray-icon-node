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

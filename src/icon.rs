use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[derive(Clone)]
pub struct Icon {
    pub(crate) inner: tray_icon::Icon,
    pub(crate) rgba: Vec<u8>,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[napi]
impl Icon {
    #[napi(factory)]
    pub fn from_path(path: String) -> Result<Self> {
        let img = image::open(&path)
            .map_err(|e| Error::from_reason(format!("Failed to open image {}: {}", path, e)))?
            .to_rgba8();
        let (width, height) = img.dimensions();
        let rgba = img.into_raw();
        let icon = tray_icon::Icon::from_rgba(rgba.clone(), width, height)
            .map_err(|e| Error::from_reason(format!("Failed to create icon: {}", e)))?;
        Ok(Self {
            inner: icon,
            rgba,
            width,
            height,
        })
    }

    #[napi(factory)]
    pub fn from_rgba(rgba: Buffer, width: u32, height: u32) -> Result<Self> {
        let rgba_vec = rgba.to_vec();
        let icon = tray_icon::Icon::from_rgba(rgba_vec.clone(), width, height)
            .map_err(|e| Error::from_reason(format!("Failed to create icon from RGBA: {}", e)))?;
        Ok(Self {
            inner: icon,
            rgba: rgba_vec,
            width,
            height,
        })
    }
}

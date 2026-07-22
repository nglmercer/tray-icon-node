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
    /// Creates an icon from an image file at the given path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or decoded as an image.
    #[napi(factory)]
    pub fn from_path(path: String) -> Result<Self> {
        let img = image::open(&path)
            .map_err(|e| Error::from_reason(format!("Failed to open image {path}: {e}")))?
            .to_rgba8();
        let (width, height) = img.dimensions();
        let rgba = img.into_raw();
        let icon = tray_icon::Icon::from_rgba(rgba.clone(), width, height)
            .map_err(|e| Error::from_reason(format!("Failed to create icon: {e}")))?;
        Ok(Self {
            inner: icon,
            rgba,
            width,
            height,
        })
    }

    /// Creates an icon from raw RGBA pixel data.
    ///
    /// # Errors
    ///
    /// Returns an error if the RGBA data length does not match `width * height * 4`
    /// or the OS fails to create the icon.
    #[napi(factory)]
    pub fn from_rgba(rgba: Buffer, width: u32, height: u32) -> Result<Self> {
        let rgba_vec = rgba.to_vec();
        let expected_len = width as usize * height as usize * 4;
        if rgba_vec.len() != expected_len {
            return Err(Error::from_reason(format!(
                "RGBA buffer length {} does not match expected size {} ({}×{}×4)",
                rgba_vec.len(),
                expected_len,
                width,
                height
            )));
        }
        let icon = tray_icon::Icon::from_rgba(rgba_vec.clone(), width, height)
            .map_err(|e| Error::from_reason(format!("Failed to create icon from RGBA: {e}")))?;
        Ok(Self {
            inner: icon,
            rgba: rgba_vec,
            width,
            height,
        })
    }
}

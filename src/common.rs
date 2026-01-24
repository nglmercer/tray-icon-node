use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl From<tray_icon::Rect> for Rect {
    fn from(rect: tray_icon::Rect) -> Self {
        Self {
            x: rect.position.x,
            y: rect.position.y,
            width: rect.size.width as f64,
            height: rect.size.height as f64,
        }
    }
}

#[napi]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl From<tray_icon::MouseButton> for MouseButton {
    fn from(button: tray_icon::MouseButton) -> Self {
        match button {
            tray_icon::MouseButton::Left => MouseButton::Left,
            tray_icon::MouseButton::Right => MouseButton::Right,
            tray_icon::MouseButton::Middle => MouseButton::Middle,
        }
    }
}

#[napi]
pub enum MouseButtonState {
    Up,
    Down,
}

impl From<tray_icon::MouseButtonState> for MouseButtonState {
    fn from(state: tray_icon::MouseButtonState) -> Self {
        match state {
            tray_icon::MouseButtonState::Up => MouseButtonState::Up,
            tray_icon::MouseButtonState::Down => MouseButtonState::Down,
        }
    }
}

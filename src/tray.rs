use crate::common::{MouseButton, MouseButtonState, Rect};
use crate::icon::Icon;
use crate::menu::Menu;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use tray_icon::{
    TrayIcon as RawTrayIcon, TrayIconBuilder as RawTrayIconBuilder,
    TrayIconEvent as RawTrayIconEvent,
};

#[napi(object)]
pub struct TrayIconEvent {
    pub event_type: String,
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub icon_rect: Rect,
    pub button: MouseButton,
    pub button_state: MouseButtonState,
}

impl From<RawTrayIconEvent> for TrayIconEvent {
    fn from(event: RawTrayIconEvent) -> Self {
        match event {
            RawTrayIconEvent::Click {
                id,
                position,
                rect,
                button,
                button_state,
            } => Self {
                event_type: "click".to_string(),
                id: id.0,
                x: position.x,
                y: position.y,
                icon_rect: rect.into(),
                button: button.into(),
                button_state: button_state.into(),
            },
            RawTrayIconEvent::DoubleClick {
                id,
                position,
                rect,
                button,
            } => Self {
                event_type: "double-click".to_string(),
                id: id.0,
                x: position.x,
                y: position.y,
                icon_rect: rect.into(),
                button: button.into(),
                button_state: tray_icon::MouseButtonState::Down.into(),
            },
            RawTrayIconEvent::Enter { id, position, rect } => Self {
                event_type: "enter".to_string(),
                id: id.0,
                x: position.x,
                y: position.y,
                icon_rect: rect.into(),
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
            },
            RawTrayIconEvent::Move { id, position, rect } => Self {
                event_type: "move".to_string(),
                id: id.0,
                x: position.x,
                y: position.y,
                icon_rect: rect.into(),
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
            },
            RawTrayIconEvent::Leave { id, position, rect } => Self {
                event_type: "leave".to_string(),
                id: id.0,
                x: position.x,
                y: position.y,
                icon_rect: rect.into(),
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
            },
            _ => Self {
                event_type: "unknown".to_string(),
                id: "".to_string(),
                x: 0.0,
                y: 0.0,
                icon_rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                },
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
            },
        }
    }
}

#[napi]
pub fn poll_tray_events() -> Option<TrayIconEvent> {
    tray_icon::TrayIconEvent::receiver()
        .try_recv()
        .ok()
        .map(|e| e.into())
}

#[napi]
pub struct TrayIcon(Option<RawTrayIcon>);

#[napi]
impl TrayIcon {
    #[napi]
    pub fn set_icon(&mut self, icon: Option<&Icon>) -> Result<()> {
        if let Some(tray) = &self.0 {
            tray.set_icon(icon.map(|i| i.inner.clone()))
                .map_err(|e| Error::from_reason(format!("Failed to set icon: {}", e)))?;
        }
        Ok(())
    }

    #[napi]
    pub fn set_tooltip(&mut self, tooltip: Option<String>) -> Result<()> {
        if let Some(tray) = &self.0 {
            let _ = tray.set_tooltip(tooltip);
        }
        Ok(())
    }

    #[napi]
    pub fn set_title(&mut self, title: Option<String>) -> Result<()> {
        if let Some(tray) = &self.0 {
            let _ = tray.set_title(title);
        }
        Ok(())
    }

    #[napi]
    pub fn set_visible(&mut self, visible: bool) -> Result<()> {
        if let Some(tray) = &self.0 {
            let _ = tray.set_visible(visible);
        }
        Ok(())
    }
}

#[napi]
#[derive(Clone)]
pub struct TrayIconBuilder {
    icon: Option<tray_icon::Icon>,
    tooltip: Option<String>,
    title: Option<String>,
    menu: Option<tray_icon::menu::Menu>,
}

#[napi]
impl TrayIconBuilder {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            icon: None,
            tooltip: None,
            title: None,
            menu: None,
        }
    }

    #[napi]
    pub fn with_icon(&mut self, icon: &Icon) -> TrayIconBuilder {
        self.icon = Some(icon.inner.clone());
        self.clone()
    }

    #[napi]
    pub fn with_tooltip(&mut self, tooltip: String) -> TrayIconBuilder {
        self.tooltip = Some(tooltip);
        self.clone()
    }

    #[napi]
    pub fn with_title(&mut self, title: String) -> TrayIconBuilder {
        self.title = Some(title);
        self.clone()
    }

    #[napi]
    pub fn with_menu(&mut self, menu: &Menu) -> TrayIconBuilder {
        self.menu = Some(menu.0.clone());
        self.clone()
    }

    #[napi]
    pub fn build(&self) -> Result<TrayIcon> {
        let mut builder = RawTrayIconBuilder::new();
        if let Some(icon) = &self.icon {
            builder = builder.with_icon(icon.clone());
        }
        if let Some(tooltip) = &self.tooltip {
            builder = builder.with_tooltip(tooltip);
        }
        if let Some(title) = &self.title {
            builder = builder.with_title(title);
        }
        if let Some(menu) = &self.menu {
            builder = builder.with_menu(Box::new(menu.clone()));
        }

        let tray = builder
            .build()
            .map_err(|e| Error::from_reason(format!("Failed to build tray icon: {}", e)))?;
        Ok(TrayIcon(Some(tray)))
    }
}

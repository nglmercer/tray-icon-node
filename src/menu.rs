use crate::icon::Icon;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tray_icon::menu as tray_menu;

pub enum AnyMenuItem {
    Standard(tray_menu::MenuItem),
    Check(tray_menu::CheckMenuItem),
    Icon(tray_menu::IconMenuItem),
    Submenu(tray_menu::Submenu),
}

unsafe impl Send for AnyMenuItem {}
unsafe impl Sync for AnyMenuItem {}

#[napi]
pub struct Menu {
    pub(crate) inner: tray_menu::Menu,
    pub(crate) registry: Arc<Mutex<HashMap<String, AnyMenuItem>>>,
}

#[napi]
impl Menu {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: tray_menu::Menu::new(),
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    fn register(&self, id: String, item: AnyMenuItem) {
        let mut reg = self.registry.lock().unwrap();
        reg.insert(id, item);
    }
    #[napi]
    pub fn append_check_menu_item(&self, item: &CheckMenuItem, id: String) -> Result<()> {
        self.inner
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))?;

        self.register(id, AnyMenuItem::Check(item.0.clone()));
        Ok(())
    }
    #[napi]
    pub fn append_menu_item(&self, item: &MenuItem, id: Option<String>) -> Result<()> {
        self.inner
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))?;

        if let Some(id_str) = id {
            self.register(id_str, AnyMenuItem::Standard(item.0.clone()));
        }
        Ok(())
    }
    #[napi]
    pub fn append_submenu(&self, item: &Submenu, id: Option<String>) -> Result<()> {
        self.inner
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))?;

        if let Some(id_str) = id {
            self.register(id_str, AnyMenuItem::Submenu(item.0.clone()));
        }
        Ok(())
    }

    #[napi]
    pub fn append_icon_menu_item(&self, item: &IconMenuItem, id: String) -> Result<()> {
        self.inner
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))?;

        self.register(id, AnyMenuItem::Icon(item.0.clone()));
        Ok(())
    }

    #[napi]
    pub fn append_predefined_menu_item(&self, item: &PredefinedMenuItem) -> Result<()> {
        self.inner
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))
    }
    #[napi]
    pub fn is_checked(&self, id: String) -> bool {
        let reg = self.registry.lock().unwrap();
        if let Some(AnyMenuItem::Check(item)) = reg.get(&id) {
            return item.is_checked();
        }
        false
    }
    #[napi]
    pub fn toggle_check(&self, id: String) -> bool {
        let reg = self.registry.lock().unwrap();
        if let Some(AnyMenuItem::Check(item)) = reg.get(&id) {
            let new_state = !item.is_checked();
            item.set_checked(new_state);
            return new_state;
        }
        false
    }

    #[napi]
    pub fn set_text(&self, id: String, text: String) {
        let reg = self.registry.lock().unwrap();
        if let Some(any_item) = reg.get(&id) {
            match any_item {
                AnyMenuItem::Standard(i) => i.set_text(text),
                AnyMenuItem::Check(i) => i.set_text(text),
                AnyMenuItem::Icon(i) => i.set_text(text),
                AnyMenuItem::Submenu(i) => i.set_text(text),
            }
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
pub struct MenuItem(pub(crate) tray_menu::MenuItem);

#[napi]
impl MenuItem {
    #[napi]
    pub fn set_text(&self, text: String) {
        self.0.set_text(text);
    }

    #[napi]
    pub fn set_enabled(&self, enabled: bool) {
        self.0.set_enabled(enabled);
    }
}

#[napi]
#[derive(Clone)]
pub struct MenuItemBuilder {
    text: String,
    enabled: bool,
    id: Option<String>,
}

#[napi]
impl MenuItemBuilder {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            text: String::new(),
            enabled: true,
            id: None,
        }
    }

    #[napi]
    pub fn with_text(&mut self, text: String) -> MenuItemBuilder {
        self.text = text;
        self.clone()
    }

    #[napi]
    pub fn with_enabled(&mut self, enabled: bool) -> MenuItemBuilder {
        self.enabled = enabled;
        self.clone()
    }

    #[napi]
    pub fn with_id(&mut self, id: String) -> MenuItemBuilder {
        self.id = Some(id);
        self.clone()
    }

    #[napi]
    pub fn build(&self) -> Result<MenuItem> {
        let item = if let Some(id) = &self.id {
            tray_menu::MenuItem::with_id(
                tray_menu::MenuId(id.clone()),
                &self.text,
                self.enabled,
                None,
            )
        } else {
            tray_menu::MenuItem::new(&self.text, self.enabled, None)
        };
        Ok(MenuItem(item))
    }
}

impl Default for MenuItemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
pub struct CheckMenuItem(pub(crate) tray_menu::CheckMenuItem);

#[napi]
#[derive(Clone)]
pub struct CheckMenuItemBuilder {
    text: String,
    enabled: bool,
    checked: bool,
    id: Option<String>,
}

#[napi]
impl CheckMenuItemBuilder {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            text: String::new(),
            enabled: true,
            checked: false,
            id: None,
        }
    }

    #[napi]
    pub fn with_text(&mut self, text: String) -> CheckMenuItemBuilder {
        self.text = text;
        self.clone()
    }

    #[napi]
    pub fn with_enabled(&mut self, enabled: bool) -> CheckMenuItemBuilder {
        self.enabled = enabled;
        self.clone()
    }

    #[napi]
    pub fn with_checked(&mut self, checked: bool) -> CheckMenuItemBuilder {
        self.checked = checked;
        self.clone()
    }

    #[napi]
    pub fn with_id(&mut self, id: String) -> CheckMenuItemBuilder {
        self.id = Some(id);
        self.clone()
    }

    #[napi]
    pub fn build(&self) -> Result<CheckMenuItem> {
        let item = if let Some(id) = &self.id {
            tray_menu::CheckMenuItem::with_id(
                tray_menu::MenuId(id.clone()),
                &self.text,
                self.enabled,
                self.checked,
                None,
            )
        } else {
            tray_menu::CheckMenuItem::new(&self.text, self.enabled, self.checked, None)
        };
        Ok(CheckMenuItem(item))
    }
}

impl Default for CheckMenuItemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
pub struct Submenu(pub(crate) tray_menu::Submenu);

#[napi]
impl CheckMenuItem {
    #[napi]
    pub fn is_checked(&self) -> bool {
        self.0.is_checked()
    }

    #[napi]
    pub fn set_checked(&self, checked: bool) {
        self.0.set_checked(checked);
    }
}

#[napi]
impl Submenu {
    #[napi]
    pub fn append_menu_item(&self, item: &MenuItem) -> Result<()> {
        self.0
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))
    }

    #[napi]
    pub fn append_submenu(&self, item: &Submenu) -> Result<()> {
        self.0
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))
    }

    #[napi]
    pub fn append_check_menu_item(&self, item: &CheckMenuItem) -> Result<()> {
        self.0
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))
    }

    #[napi]
    pub fn append_icon_menu_item(&self, item: &IconMenuItem) -> Result<()> {
        self.0
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))
    }

    #[napi]
    pub fn append_predefined_menu_item(&self, item: &PredefinedMenuItem) -> Result<()> {
        self.0
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))
    }
}

#[napi]
#[derive(Clone)]
pub struct SubmenuBuilder {
    text: String,
    enabled: bool,
}

#[napi]
impl SubmenuBuilder {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            text: String::new(),
            enabled: true,
        }
    }

    #[napi]
    pub fn with_text(&mut self, text: String) -> SubmenuBuilder {
        self.text = text;
        self.clone()
    }

    #[napi]
    pub fn with_enabled(&mut self, enabled: bool) -> SubmenuBuilder {
        self.enabled = enabled;
        self.clone()
    }

    #[napi]
    pub fn build(&self) -> Result<Submenu> {
        Ok(Submenu(tray_menu::Submenu::new(&self.text, self.enabled)))
    }
}

impl Default for SubmenuBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
pub struct PredefinedMenuItem(pub(crate) tray_menu::PredefinedMenuItem);

#[napi]
impl PredefinedMenuItem {
    #[napi]
    pub fn separator() -> Self {
        Self(tray_menu::PredefinedMenuItem::separator())
    }
}

#[napi]
pub struct IconMenuItem(pub(crate) tray_menu::IconMenuItem);

#[napi]
impl IconMenuItem {
    #[napi]
    pub fn set_text(&self, text: String) {
        self.0.set_text(text);
    }

    #[napi]
    pub fn set_enabled(&self, enabled: bool) {
        self.0.set_enabled(enabled);
    }
}

#[napi]
#[derive(Clone)]
pub struct IconMenuItemBuilder {
    text: String,
    enabled: bool,
    icon: Option<tray_menu::Icon>,
    id: Option<String>,
}

#[napi]
impl IconMenuItemBuilder {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            text: String::new(),
            enabled: true,
            icon: None,
            id: None,
        }
    }

    #[napi]
    pub fn with_text(&mut self, text: String) -> IconMenuItemBuilder {
        self.text = text;
        self.clone()
    }

    #[napi]
    pub fn with_enabled(&mut self, enabled: bool) -> IconMenuItemBuilder {
        self.enabled = enabled;
        self.clone()
    }

    #[napi]
    pub fn with_icon(&mut self, icon: &Icon) -> Result<IconMenuItemBuilder> {
        let tray_icon = tray_menu::Icon::from_rgba(icon.rgba.clone(), icon.width, icon.height)
            .map_err(|e| Error::from_reason(format!("Failed to create menu icon: {e}")))?;
        self.icon = Some(tray_icon);
        Ok(self.clone())
    }

    #[napi]
    pub fn with_id(&mut self, id: String) -> IconMenuItemBuilder {
        self.id = Some(id);
        self.clone()
    }

    #[napi]
    pub fn build(&self) -> Result<IconMenuItem> {
        let icon = self
            .icon
            .clone()
            .ok_or_else(|| Error::from_reason("Icon is required".to_string()))?;
        let item = if let Some(id) = &self.id {
            tray_menu::IconMenuItem::with_id(
                tray_menu::MenuId(id.clone()),
                &self.text,
                self.enabled,
                Some(icon),
                None,
            )
        } else {
            tray_menu::IconMenuItem::new(&self.text, self.enabled, Some(icon), None)
        };
        Ok(IconMenuItem(item))
    }
}

impl Default for IconMenuItemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[napi(object)]
pub struct MenuEvent {
    pub id: String,
}

#[napi]
pub fn poll_menu_events() -> Option<MenuEvent> {
    tray_menu::MenuEvent::receiver()
        .try_recv()
        .ok()
        .map(|e| MenuEvent { id: e.id.0 })
}

#[napi(object)]
pub struct AboutMetadata {
    pub name: Option<String>,
    pub version: Option<String>,
    pub copyright: Option<String>,
    pub authors: Option<Vec<String>>,
    pub website: Option<String>,
    pub website_label: Option<String>,
    pub comments: Option<String>,
}

#[napi]
#[derive(Clone)]
pub struct AboutMetadataBuilder {
    pub name: Option<String>,
    pub version: Option<String>,
    pub copyright: Option<String>,
    pub authors: Option<Vec<String>>,
    pub website: Option<String>,
    pub website_label: Option<String>,
    pub comments: Option<String>,
}

#[napi]
impl AboutMetadataBuilder {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            name: None,
            version: None,
            copyright: None,
            authors: None,
            website: None,
            website_label: None,
            comments: None,
        }
    }

    #[napi]
    pub fn with_name(&mut self, name: String) -> AboutMetadataBuilder {
        self.name = Some(name);
        self.clone()
    }

    #[napi]
    pub fn with_version(&mut self, version: String) -> AboutMetadataBuilder {
        self.version = Some(version);
        self.clone()
    }

    #[napi]
    pub fn with_copyright(&mut self, copyright: String) -> AboutMetadataBuilder {
        self.copyright = Some(copyright);
        self.clone()
    }

    #[napi]
    pub fn with_authors(&mut self, authors: Vec<String>) -> AboutMetadataBuilder {
        self.authors = Some(authors);
        self.clone()
    }

    #[napi]
    pub fn with_website(&mut self, website: String) -> AboutMetadataBuilder {
        self.website = Some(website);
        self.clone()
    }

    #[napi]
    pub fn with_website_label(&mut self, website_label: String) -> AboutMetadataBuilder {
        self.website_label = Some(website_label);
        self.clone()
    }

    #[napi]
    pub fn with_comments(&mut self, comments: String) -> AboutMetadataBuilder {
        self.comments = Some(comments);
        self.clone()
    }

    #[napi]
    pub fn build(&self) -> AboutMetadata {
        AboutMetadata {
            name: self.name.clone(),
            version: self.version.clone(),
            copyright: self.copyright.clone(),
            authors: self.authors.clone(),
            website: self.website.clone(),
            website_label: self.website_label.clone(),
            comments: self.comments.clone(),
        }
    }
}

impl Default for AboutMetadataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

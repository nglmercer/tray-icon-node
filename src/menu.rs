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
    Submenu(Submenu),
}

// SAFETY: All variants of `AnyMenuItem` wrap types from the `tray-icon` crate,
// which guarantees `Send + Sync` safety across all supported platforms (Windows, macOS, Linux).
// The underlying OS handles (HMENU, NSMenu, GtkMenu) are reference-counted and safe to move
// between threads for read-only operations. Mutable access is protected by the `Mutex` in `Menu`.
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for AnyMenuItem {}
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Sync for AnyMenuItem {}

#[inline]
fn lock_registry(
    reg: &Mutex<HashMap<String, AnyMenuItem>>,
) -> std::sync::MutexGuard<'_, HashMap<String, AnyMenuItem>> {
    reg.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

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
        let mut reg = lock_registry(&self.registry);
        reg.insert(id, item);
    }

    /// Appends a checkable menu item to this menu.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS menu operation fails.
    #[napi]
    pub fn append_check_menu_item(&self, item: &CheckMenuItem, id: String) -> Result<()> {
        self.inner
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))?;

        self.register(id, AnyMenuItem::Check(item.0.clone()));
        Ok(())
    }

    /// Appends a standard menu item to this menu.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS menu operation fails.
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

    /// Appends a submenu to this menu.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS menu operation fails.
    #[napi]
    pub fn append_submenu(&self, item: &Submenu, id: Option<String>) -> Result<()> {
        self.inner
            .append(&item.inner)
            .map_err(|e| Error::from_reason(format!("{e}")))?;

        if let Some(id_str) = id {
            self.register(id_str, AnyMenuItem::Submenu(item.clone()));
        }
        Ok(())
    }

    /// Appends an icon menu item to this menu.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS menu operation fails.
    #[napi]
    pub fn append_icon_menu_item(&self, item: &IconMenuItem, id: String) -> Result<()> {
        self.inner
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))?;

        self.register(id, AnyMenuItem::Icon(item.0.clone()));
        Ok(())
    }

    /// Appends a predefined menu item (e.g., separator) to this menu.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS menu operation fails.
    #[napi]
    pub fn append_predefined_menu_item(&self, item: &PredefinedMenuItem) -> Result<()> {
        self.inner
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))
    }

    #[napi]
    pub fn is_checked(&self, id: String) -> bool {
        let reg = lock_registry(&self.registry);
        if let Some(AnyMenuItem::Check(item)) = reg.get(&id) {
            return item.is_checked();
        }
        for item in reg.values() {
            if let AnyMenuItem::Submenu(submenu) = item {
                if submenu.is_checked(id.clone()) {
                    return true;
                }
            }
        }
        false
    }

    #[napi]
    pub fn toggle_check(&self, id: String) -> bool {
        let reg = lock_registry(&self.registry);
        if let Some(AnyMenuItem::Check(item)) = reg.get(&id) {
            return item.is_checked();
        }
        for item in reg.values() {
            if let AnyMenuItem::Submenu(submenu) = item {
                if submenu.has_id(&id) {
                    return submenu.toggle_check(id.clone());
                }
            }
        }
        false
    }

    #[napi]
    pub fn set_text(&self, id: String, text: String) {
        let reg = lock_registry(&self.registry);
        if let Some(any_item) = reg.get(&id) {
            match any_item {
                AnyMenuItem::Standard(i) => i.set_text(text),
                AnyMenuItem::Check(i) => i.set_text(text),
                AnyMenuItem::Icon(i) => i.set_text(text),
                AnyMenuItem::Submenu(i) => i.inner.set_text(text),
            }
            return;
        }
        for item in reg.values() {
            if let AnyMenuItem::Submenu(submenu) = item {
                if submenu.has_id(&id) {
                    submenu.set_text(id, text);
                    return;
                }
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
    #[must_use]
    pub fn with_text(&mut self, text: String) -> Self {
        self.text = text;
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_enabled(&mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_id(&mut self, id: String) -> Self {
        self.id = Some(id);
        self.clone()
    }

    /// Builds the menu item.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS fails to create the menu item.
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
    #[must_use]
    pub fn with_text(&mut self, text: String) -> Self {
        self.text = text;
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_enabled(&mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_checked(&mut self, checked: bool) -> Self {
        self.checked = checked;
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_id(&mut self, id: String) -> Self {
        self.id = Some(id);
        self.clone()
    }

    /// Builds the check menu item.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS fails to create the menu item.
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
#[derive(Clone)]
pub struct Submenu {
    pub(crate) inner: tray_menu::Submenu,
    pub(crate) registry: Arc<Mutex<HashMap<String, AnyMenuItem>>>,
}

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

impl Submenu {
    fn register(&self, id: String, item: AnyMenuItem) {
        let mut reg = lock_registry(&self.registry);
        reg.insert(id, item);
    }

    fn has_id(&self, id: &str) -> bool {
        let reg = lock_registry(&self.registry);
        if reg.contains_key(id) {
            return true;
        }
        for item in reg.values() {
            if let AnyMenuItem::Submenu(submenu) = item {
                if submenu.has_id(id) {
                    return true;
                }
            }
        }
        false
    }

    fn set_text(&self, id: String, text: String) {
        let reg = lock_registry(&self.registry);
        if let Some(any_item) = reg.get(&id) {
            match any_item {
                AnyMenuItem::Standard(i) => i.set_text(text),
                AnyMenuItem::Check(i) => i.set_text(text),
                AnyMenuItem::Icon(i) => i.set_text(text),
                AnyMenuItem::Submenu(i) => i.inner.set_text(text),
            }
            return;
        }
        for item in reg.values() {
            if let AnyMenuItem::Submenu(submenu) = item {
                if submenu.has_id(&id) {
                    submenu.set_text(id, text);
                    return;
                }
            }
        }
    }
}

#[napi]
impl Submenu {
    /// Appends a standard menu item to this submenu.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS menu operation fails.
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

    /// Appends a nested submenu to this submenu.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS menu operation fails.
    #[allow(clippy::use_self)]
    #[napi]
    pub fn append_submenu(&self, item: &Submenu, id: Option<String>) -> Result<()> {
        self.inner
            .append(&item.inner)
            .map_err(|e| Error::from_reason(format!("{e}")))?;

        if let Some(id_str) = id {
            self.register(id_str, AnyMenuItem::Submenu(item.clone()));
        }
        Ok(())
    }

    /// Appends a checkable menu item to this submenu.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS menu operation fails.
    #[napi]
    pub fn append_check_menu_item(&self, item: &CheckMenuItem, id: String) -> Result<()> {
        self.inner
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))?;

        self.register(id, AnyMenuItem::Check(item.0.clone()));
        Ok(())
    }

    /// Appends an icon menu item to this submenu.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS menu operation fails.
    #[napi]
    pub fn append_icon_menu_item(&self, item: &IconMenuItem, id: Option<String>) -> Result<()> {
        self.inner
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))?;

        if let Some(id_str) = id {
            self.register(id_str, AnyMenuItem::Icon(item.0.clone()));
        }
        Ok(())
    }

    /// Appends a predefined menu item (e.g., separator) to this submenu.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS menu operation fails.
    #[napi]
    pub fn append_predefined_menu_item(&self, item: &PredefinedMenuItem) -> Result<()> {
        self.inner
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{e}")))
    }

    #[napi]
    pub fn is_checked(&self, id: String) -> bool {
        let reg = lock_registry(&self.registry);
        if let Some(AnyMenuItem::Check(item)) = reg.get(&id) {
            return item.is_checked();
        }
        for item in reg.values() {
            if let AnyMenuItem::Submenu(submenu) = item {
                if submenu.is_checked(id.clone()) {
                    return true;
                }
            }
        }
        false
    }

    #[napi]
    pub fn toggle_check(&self, id: String) -> bool {
        let reg = lock_registry(&self.registry);
        if let Some(AnyMenuItem::Check(item)) = reg.get(&id) {
            return item.is_checked();
        }
        for item in reg.values() {
            if let AnyMenuItem::Submenu(submenu) = item {
                if submenu.has_id(&id) {
                    return submenu.toggle_check(id.clone());
                }
            }
        }
        false
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
    #[must_use]
    pub fn with_text(&mut self, text: String) -> Self {
        self.text = text;
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_enabled(&mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self.clone()
    }

    /// Builds the submenu.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying OS fails to create the submenu.
    #[napi]
    pub fn build(&self) -> Result<Submenu> {
        Ok(Submenu {
            inner: tray_menu::Submenu::new(&self.text, self.enabled),
            registry: Arc::new(Mutex::new(HashMap::new())),
        })
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
    #[must_use]
    pub fn with_text(&mut self, text: String) -> Self {
        self.text = text;
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_enabled(&mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self.clone()
    }

    /// Sets the icon for this menu item.
    ///
    /// # Errors
    ///
    /// Returns an error if the icon data is invalid or the OS fails to create the icon.
    #[napi]
    pub fn with_icon(&mut self, icon: &Icon) -> Result<Self> {
        let tray_icon = tray_menu::Icon::from_rgba(icon.rgba.clone(), icon.width, icon.height)
            .map_err(|e| Error::from_reason(format!("Failed to create menu icon: {e}")))?;
        self.icon = Some(tray_icon);
        Ok(self.clone())
    }

    #[napi]
    #[must_use]
    pub fn with_id(&mut self, id: String) -> Self {
        self.id = Some(id);
        self.clone()
    }

    /// Builds the icon menu item.
    ///
    /// # Errors
    ///
    /// Returns an error if no icon was set or the OS fails to create the menu item.
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
    #[must_use]
    pub fn with_name(&mut self, name: String) -> Self {
        self.name = Some(name);
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_version(&mut self, version: String) -> Self {
        self.version = Some(version);
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_copyright(&mut self, copyright: String) -> Self {
        self.copyright = Some(copyright);
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_authors(&mut self, authors: Vec<String>) -> Self {
        self.authors = Some(authors);
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_website(&mut self, website: String) -> Self {
        self.website = Some(website);
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_website_label(&mut self, website_label: String) -> Self {
        self.website_label = Some(website_label);
        self.clone()
    }

    #[napi]
    #[must_use]
    pub fn with_comments(&mut self, comments: String) -> Self {
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

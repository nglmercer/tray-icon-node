use crate::icon::Icon;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use tray_icon::menu as tray_menu;

#[napi]
pub struct Menu(pub(crate) tray_menu::Menu);

#[napi]
impl Menu {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self(tray_menu::Menu::new())
    }

    #[napi]
    pub fn append_menu_item(&self, item: &MenuItem) -> Result<()> {
        self.0
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn append_submenu(&self, item: &Submenu) -> Result<()> {
        self.0
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn append_check_menu_item(&self, item: &CheckMenuItem) -> Result<()> {
        self.0
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn append_icon_menu_item(&self, item: &IconMenuItem) -> Result<()> {
        self.0
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn append_predefined_menu_item(&self, item: &PredefinedMenuItem) -> Result<()> {
        self.0
            .append(&item.0)
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }
}

#[napi]
pub struct MenuItem(pub(crate) tray_menu::MenuItem);

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

#[napi]
pub struct Submenu(pub(crate) tray_menu::Submenu);

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
            .map_err(|e| Error::from_reason(format!("Failed to create menu icon: {}", e)))?;
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

#[napi]
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

#[napi]
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

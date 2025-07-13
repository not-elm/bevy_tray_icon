//! Resources used by this plugin.

use bevy::asset::{Assets, Handle};
use bevy::prelude::{Deref, Image, Resource};
use tray_icon::{menu, Icon};

pub use menu_item::MenuItem;

mod menu_item;

/// After this resource is generated, the system tray is also generated.
///
/// When you change a field in this resource, the system tray is also updated.

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Resource)]
pub struct TrayIcon {
    /// The image handle of icon.
    pub icon: Option<Handle<Image>>,

    /// The text in tooltip displayed when mouse hovers over the system tray.
    pub tooltip: Option<String>,

    /// The menu displayed when right-clicking on the system tray.
    pub menu: Menu,

    #[cfg(target_os = "macos")]
    pub show_menu_on_left_click: bool,
}

impl TrayIcon {
    pub(crate) fn as_icon(&self, images: &Assets<Image>) -> Option<Icon> {
        let id = self.icon.as_ref().map(|handle| handle.id())?;
        let image = images.get(id)?;
        Icon::from_rgba(image.data.clone()?, image.width(), image.height()).ok()
    }
}

/// The system tray's menu.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Menu(Vec<MenuItem>);

impl Menu {
    /// Creates the new [`Menu`].
    #[inline]
    pub const fn new(items: Vec<MenuItem>) -> Self {
        Self(items)
    }

    /// Enables or disables the specified menu.
    pub fn set_enable(&mut self, menu_id: &str, enable: bool) -> crate::error::Result {
        for menu in self.0.iter_mut() {
            match menu {
                MenuItem::Common { id, enabled, .. } => {
                    if id.0 == menu_id {
                        *enabled = enable;
                        return Ok(());
                    }
                }
                MenuItem::SubMenu {
                    id, enabled, menu, ..
                } => {
                    if id.0 == menu_id {
                        *enabled = enable;
                        return Ok(());
                    } else if menu.set_enable(menu_id, enable).is_ok() {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
        Err(crate::error::Error::NotFoundMenu(menu_id.to_string()))
    }

    pub(crate) fn as_context(&self) -> Option<Box<dyn menu::ContextMenu>> {
        if self.0.is_empty() {
            None
        } else {
            let menu = menu::Menu::new();
            for item in self.0.iter().map(|item| item.as_dyn()) {
                menu.append(&*item).unwrap();
            }
            Some(Box::new(menu))
        }
    }
}

#[repr(transparent)]
#[derive(Deref)]
pub(crate) struct RealTrayIcon(pub(crate) tray_icon::TrayIcon);

#[cfg(test)]
mod tests {
    use crate::resource::{Menu, MenuItem};

    #[test]
    fn enable_common() {
        let mut menu = Menu::new(vec![MenuItem::common("test", "test", false, None)]);
        menu.set_enable("test", true).unwrap();
        assert!(check_enabled("test", &menu));
    }

    #[test]
    fn enable_submenu() {
        let mut menu = Menu::new(vec![
            MenuItem::common("test", "test", false, None),
            MenuItem::submenu("sub", "sub", false, Menu::default()),
        ]);
        menu.set_enable("sub", true).unwrap();
        assert!(check_enabled("sub", &menu));
    }

    #[test]
    fn enable_item_in_submenu() {
        let mut menu = Menu::new(vec![
            MenuItem::common("test", "test", false, None),
            MenuItem::submenu(
                "sub",
                "sub",
                false,
                Menu::new(vec![MenuItem::common("sub_item", "sub_item", false, None)]),
            ),
        ]);
        menu.set_enable("sub_item", true).unwrap();
        assert!(check_enabled("sub_item", &menu));
    }

    fn check_enabled(target_id: &str, menu: &Menu) -> bool {
        for item in menu.0.iter() {
            match item {
                MenuItem::SubMenu {
                    id, menu, enabled, ..
                } => {
                    if id == target_id {
                        return *enabled;
                    } else if check_enabled(target_id, menu) {
                        return true;
                    }
                }
                MenuItem::Common { id, enabled, .. } => {
                    if id == target_id {
                        return *enabled;
                    }
                }
                _ => {}
            }
        }
        false
    }
}

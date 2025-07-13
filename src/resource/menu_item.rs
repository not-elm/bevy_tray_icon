use tray_icon::menu;
use tray_icon::menu::accelerator::Accelerator;
use tray_icon::menu::{IsMenuItem, MenuId, PredefinedMenuItem, Submenu};

use crate::resource::Menu;

/// System Tray Menu Item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MenuItem {
    /// The menu common item.
    Common {
        /// Menu id.
        id: MenuId,
        /// The text displayed in the menu.
        text: String,
        /// whether you can click on this item.
        enabled: bool,
        /// A keyboard shortcut that consists of an optional combination of modifier keys (provided by Modifiers) and one key (Code).
        accelerator: Option<Accelerator>,
    },

    /// Submenu.
    SubMenu {
        /// Menu id
        id: MenuId,
        /// The text displayed in the menu.
        text: String,
        /// whether you can click on this item.
        enabled: bool,
        /// A keyboard shortcut that consists of an optional combination of modifier keys (provided by Modifiers) and one key (Code).
        menu: Menu,
    },

    /// The menu separator.
    Separator,
}

impl MenuItem {
    /// Creates the common item.
    ///
    /// ## Params
    ///
    /// * `id` - Menu-id.
    /// * `text` - The text displayed in the menu.
    /// * `enabled` - whether you can click on this item.
    /// * `accelerator` - A keyboard shortcut that consists of an optional combination of modifier keys (provided by Modifiers) and one key (Code).
    #[inline]
    pub fn common(
        id: impl Into<MenuId>,
        text: impl Into<String>,
        enabled: bool,
        accelerator: Option<Accelerator>,
    ) -> Self {
        Self::Common {
            id: id.into(),
            text: text.into(),
            enabled,
            accelerator,
        }
    }

    /// Create the separator.
    #[inline]
    pub const fn separator() -> Self {
        Self::Separator
    }

    /// Creates the submenu.
    ///
    /// ## Params
    ///
    /// * `id` - Menu-id.
    /// * `text` - The text displayed in the menu.
    /// * `enabled` - whether you can click on this item.
    /// * `menu` - Submenu
    #[inline]
    pub fn submenu(
        id: impl Into<MenuId>,
        text: impl Into<String>,
        enabled: bool,
        menu: Menu,
    ) -> Self {
        Self::SubMenu {
            id: id.into(),
            text: text.into(),
            enabled,
            menu,
        }
    }

    pub(super) fn as_dyn(&self) -> Box<dyn IsMenuItem> {
        match self {
            MenuItem::Separator => Box::new(PredefinedMenuItem::separator()),
            MenuItem::SubMenu {
                id,
                text,
                enabled,
                menu,
            } => {
                let sub_menu = Submenu::with_id(&id.0, text, *enabled);
                for item in menu.0.iter().map(|item| item.as_dyn()) {
                    sub_menu.append(&*item).unwrap();
                }
                Box::new(sub_menu)
            }
            MenuItem::Common {
                id,
                text,
                enabled,
                accelerator,
            } => {
                let item = menu::MenuItem::with_id(&id.0, text, *enabled, *accelerator);
                Box::new(item)
            }
        }
    }
}

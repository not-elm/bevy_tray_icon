//! Convert system tray events to bevy events.

use bevy::prelude::*;
use tray_icon::menu::MenuId;

/// The event fired when a menu is clicked.
#[derive(Message, Debug, Eq, PartialEq)]
pub struct MenuMessage {
    /// The id of the clicked menu.
    pub id: MenuId,
}

pub(crate) struct MenuEventPlugin;

impl Plugin for MenuEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MenuMessage>().add_systems(Update, menu_event);
    }
}

fn menu_event(mut ew: MessageWriter<MenuMessage>) {
    while let Ok(event) = tray_icon::menu::MenuEvent::receiver().try_recv() {
        ew.write(MenuMessage { id: event.id });
    }
}

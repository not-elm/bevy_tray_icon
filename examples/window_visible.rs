//! This example shows how to control the display state of an application from the system tray.
//! 
//! The window is not displayed when the application is launched.
//! 
//! You can switch the display status from the menu that appears by right-clicking on the system tray.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy_tray_icon::plugin::menu_event::MenuEvent;
use bevy_tray_icon::plugin::TrayIconPlugin;
use bevy_tray_icon::resource::{Menu, MenuItem, TrayIcon};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            TrayIconPlugin
        ))
        .add_systems(Startup, create_tray)
        .add_systems(Update, (
            menu_event,
            change_menu_enable.run_if(resource_exists::<TrayIcon>)
        ))
        .run();
}

fn create_tray(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(TrayIcon {
        icon: Some(asset_server.load("bevy.png")),
        tooltip: Some("tooltip".to_string()),
        menu: Menu::new(vec![
            MenuItem::common("visible", "visible", false, None),
            MenuItem::common("hide", "hide", true, None),
        ]),
    });
}

fn menu_event(
    mut er: EventReader<MenuEvent>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    for e in er.read() {
        match e.id.0.as_str() {
            "visible" => {
                window.single_mut().visible = true;
            }
            "hide" => {
                window.single_mut().visible = false;
            }
            _ => {}
        }
    }
}

fn change_menu_enable(
    mut tray: ResMut<TrayIcon>,
    window: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
) {
    let Ok(visible) = window.get_single().map(|window| window.visible) else {
        return;
    };
    tray.menu.set_enable("visible", !visible).unwrap();
    tray.menu.set_enable("hide", visible).unwrap();
}
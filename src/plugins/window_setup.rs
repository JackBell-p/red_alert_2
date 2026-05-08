use bevy::{prelude::*, window::WindowResolution};

pub struct WindowSetup;

impl Plugin for WindowSetup {
    fn build(&self, app: &mut App) {
        //Registration Window.
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Red Alert 2".to_string(),
                resolution: WindowResolution::new(1280, 768),
                resizable: false,
                ..default()
            }),
            ..default()
        }));
    }
}

use bevy::prelude::*;
use hex_game::editor::EditorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "HexGame Editor".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EditorPlugin)
        .run();
}

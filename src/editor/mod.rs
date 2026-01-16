mod camera;
mod grid;
mod input;

use bevy::prelude::*;

pub use camera::*;
pub use grid::*;
pub use input::*;

/// Main editor plugin
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EditorCameraPlugin,
            HexGridPlugin,
            EditorInputPlugin,
        ))
        .init_resource::<EditorState>()
        .add_systems(Startup, setup_editor);
    }
}

#[derive(Resource, Default)]
pub struct EditorState {
    pub selected_hex: Option<crate::hex::HexCoord>,
    pub brush_size: u32,
}

fn setup_editor(mut commands: Commands) {
    // Ambient light
    commands.spawn(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
        affects_lightmapped_meshes: true,
    });

    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

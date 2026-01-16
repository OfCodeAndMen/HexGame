use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::hex::HexCoord;
use super::{EditorCamera, EditorState, HexAssets, HexGrid, HexTile};

pub struct EditorInputPlugin;

impl Plugin for EditorInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveredHex>()
            .add_systems(Update, (
                raycast_hex_grid,
                update_hex_highlights,
                handle_hex_click,
            ).chain());
    }
}

#[derive(Resource, Default)]
pub struct HoveredHex {
    pub coord: Option<HexCoord>,
}

fn raycast_hex_grid(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<EditorCamera>>,
    hex_grid: Res<HexGrid>,
    mut hovered: ResMut<HoveredHex>,
) {
    let Ok(window): Result<&Window, _> = windows.single() else { return };
    let Ok((camera, camera_transform)): Result<(&Camera, &GlobalTransform), _> = camera_query.single() else { return };

    let Some(cursor_pos): Option<Vec2> = window.cursor_position() else {
        hovered.coord = None;
        return;
    };

    // Create ray from camera through cursor
    let Ok(ray): Result<Ray3d, _> = camera.viewport_to_world(camera_transform, cursor_pos) else {
        hovered.coord = None;
        return;
    };

    // Intersect with Y=0 plane (ground)
    let ground_y = 0.0;
    let t = (ground_y - ray.origin.y) / ray.direction.y;

    if t < 0.0 {
        hovered.coord = None;
        return;
    }

    let hit_point = ray.origin + ray.direction * t;
    let coord = HexCoord::from_world(hit_point, hex_grid.hex_size);

    // Only set hovered if this hex exists in the grid
    if hex_grid.tiles.contains_key(&coord) {
        hovered.coord = Some(coord);
    } else {
        hovered.coord = None;
    }
}

fn update_hex_highlights(
    hovered: Res<HoveredHex>,
    editor_state: Res<EditorState>,
    hex_assets: Res<HexAssets>,
    _hex_grid: Res<HexGrid>,
    mut tile_query: Query<(&HexTile, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    if !hovered.is_changed() && !editor_state.is_changed() {
        return;
    }

    for (tile, mut material) in tile_query.iter_mut() {
        let is_selected = editor_state.selected_hex == Some(tile.coord);
        let is_hovered = hovered.coord == Some(tile.coord);

        *material = MeshMaterial3d(if is_selected {
            hex_assets.selected_material.clone()
        } else if is_hovered {
            hex_assets.hovered_material.clone()
        } else {
            hex_assets.default_material.clone()
        });
    }
}

fn handle_hex_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    hovered: Res<HoveredHex>,
    mut editor_state: ResMut<EditorState>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        editor_state.selected_hex = hovered.coord;
    }
}

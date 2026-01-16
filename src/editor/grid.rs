use bevy::prelude::*;
use std::collections::HashMap;

use crate::hex::{HexCoord, create_hex_mesh};

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HexGrid>()
            .init_resource::<HexAssets>()
            .add_systems(Startup, setup_grid_assets)
            .add_systems(PostStartup, spawn_initial_grid);
    }
}

/// Resource containing hex mesh and material handles
#[derive(Resource, Default)]
pub struct HexAssets {
    pub hex_mesh: Handle<Mesh>,
    pub default_material: Handle<StandardMaterial>,
    pub hovered_material: Handle<StandardMaterial>,
    pub selected_material: Handle<StandardMaterial>,
}

/// Resource tracking all hex tiles in the grid
#[derive(Resource, Default)]
pub struct HexGrid {
    pub tiles: HashMap<HexCoord, Entity>,
    pub hex_size: f32,
    pub hex_height: f32,
}

impl HexGrid {
    pub fn new(hex_size: f32, hex_height: f32) -> Self {
        Self {
            tiles: HashMap::new(),
            hex_size,
            hex_height,
        }
    }
}

/// Component marking an entity as a hex tile
#[derive(Component)]
pub struct HexTile {
    pub coord: HexCoord,
    pub elevation: i32,
}

fn setup_grid_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let hex_size = 1.0;
    let hex_height = 0.2;

    let hex_mesh = meshes.add(create_hex_mesh(hex_size, hex_height));

    let default_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.5, 0.3),
        perceptual_roughness: 0.8,
        ..default()
    });

    let hovered_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.7, 0.5),
        perceptual_roughness: 0.8,
        ..default()
    });

    let selected_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.7, 0.7, 0.3),
        perceptual_roughness: 0.8,
        ..default()
    });

    commands.insert_resource(HexAssets {
        hex_mesh,
        default_material,
        hovered_material,
        selected_material,
    });

    commands.insert_resource(HexGrid::new(hex_size, hex_height));
}

fn spawn_initial_grid(
    mut commands: Commands,
    hex_assets: Res<HexAssets>,
    mut hex_grid: ResMut<HexGrid>,
) {
    let grid_radius = 5;

    for q in -grid_radius..=grid_radius {
        for r in (-grid_radius).max(-q - grid_radius)..=grid_radius.min(-q + grid_radius) {
            let coord = HexCoord::new(q, r);
            let world_pos = coord.to_world(hex_grid.hex_size);

            let entity = commands.spawn((
                Mesh3d(hex_assets.hex_mesh.clone()),
                MeshMaterial3d(hex_assets.default_material.clone()),
                Transform::from_translation(world_pos),
                HexTile {
                    coord,
                    elevation: 0,
                },
            )).id();

            hex_grid.tiles.insert(coord, entity);
        }
    }
}

/// Spawn a new hex tile at the given coordinate
pub fn spawn_hex_tile(
    commands: &mut Commands,
    hex_assets: &HexAssets,
    hex_grid: &mut HexGrid,
    coord: HexCoord,
    elevation: i32,
) -> Entity {
    let mut world_pos = coord.to_world(hex_grid.hex_size);
    world_pos.y = elevation as f32 * hex_grid.hex_height;

    let entity = commands.spawn((
        Mesh3d(hex_assets.hex_mesh.clone()),
        MeshMaterial3d(hex_assets.default_material.clone()),
        Transform::from_translation(world_pos),
        HexTile { coord, elevation },
    )).id();

    hex_grid.tiles.insert(coord, entity);
    entity
}

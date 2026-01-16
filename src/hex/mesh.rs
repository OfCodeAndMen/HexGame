use bevy::prelude::*;
use bevy::mesh::{Indices, PrimitiveTopology};

/// Creates a flat-top hexagon mesh
pub fn create_hex_mesh(size: f32, height: f32) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // Generate the 6 corners of a flat-top hexagon
    let corners: Vec<Vec3> = (0..6)
        .map(|i| {
            let angle = std::f32::consts::PI / 3.0 * i as f32;
            Vec3::new(size * angle.cos(), 0.0, size * angle.sin())
        })
        .collect();

    // Top face
    let top_center_idx = 0u32;
    positions.push([0.0, height, 0.0]);
    normals.push([0.0, 1.0, 0.0]);
    uvs.push([0.5, 0.5]);

    for corner in &corners {
        positions.push([corner.x, height, corner.z]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([(corner.x / size + 1.0) / 2.0, (corner.z / size + 1.0) / 2.0]);
    }

    // Top face triangles
    for i in 0..6 {
        indices.push(top_center_idx);
        indices.push(top_center_idx + 1 + i);
        indices.push(top_center_idx + 1 + (i + 1) % 6);
    }

    // Bottom face
    let bottom_center_idx = positions.len() as u32;
    positions.push([0.0, 0.0, 0.0]);
    normals.push([0.0, -1.0, 0.0]);
    uvs.push([0.5, 0.5]);

    for corner in &corners {
        positions.push([corner.x, 0.0, corner.z]);
        normals.push([0.0, -1.0, 0.0]);
        uvs.push([(corner.x / size + 1.0) / 2.0, (corner.z / size + 1.0) / 2.0]);
    }

    // Bottom face triangles (reversed winding)
    for i in 0..6 {
        indices.push(bottom_center_idx);
        indices.push(bottom_center_idx + 1 + (i + 1) % 6);
        indices.push(bottom_center_idx + 1 + i);
    }

    // Side faces
    let side_start_idx = positions.len() as u32;
    for i in 0..6 {
        let next = (i + 1) % 6;
        let c1 = corners[i];
        let c2 = corners[next];

        // Calculate normal for this side
        let normal = Vec3::new(c1.x + c2.x, 0.0, c1.z + c2.z).normalize();

        // Four vertices per side
        positions.push([c1.x, height, c1.z]);
        positions.push([c2.x, height, c2.z]);
        positions.push([c2.x, 0.0, c2.z]);
        positions.push([c1.x, 0.0, c1.z]);

        for _ in 0..4 {
            normals.push([normal.x, normal.y, normal.z]);
        }

        uvs.push([0.0, 1.0]);
        uvs.push([1.0, 1.0]);
        uvs.push([1.0, 0.0]);
        uvs.push([0.0, 0.0]);

        let base = side_start_idx + (i as u32 * 4);
        indices.push(base);
        indices.push(base + 1);
        indices.push(base + 2);
        indices.push(base);
        indices.push(base + 2);
        indices.push(base + 3);
    }

    Mesh::new(PrimitiveTopology::TriangleList, default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices))
}

/// Creates a flat hexagon outline for grid display
pub fn create_hex_outline(size: f32) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // Generate the 6 corners + back to first for line loop
    for i in 0..=6 {
        let angle = std::f32::consts::PI / 3.0 * (i % 6) as f32;
        positions.push([size * angle.cos(), 0.01, size * angle.sin()]);
        normals.push([0.0, 1.0, 0.0]);
    }

    // Line indices
    for i in 0..6 {
        indices.push(i);
        indices.push(i + 1);
    }

    Mesh::new(PrimitiveTopology::LineList, default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(indices))
}

use bevy::prelude::*;

const PIXEL_SIZE: f32 = 1.0;
const PITCH: f32 = 0.3;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_scene)
        .add_startup_system(setup_drawing)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 5000000.0,
            range: 1000.0,
            radius: 75.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 100.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 200.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Draw origin
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Torus {
            radius: 0.5,
            ring_radius: 0.1,
            subdivisions_segments: 20,
            subdivisions_sides: 20,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            metallic: 1.0,
            perceptual_roughness: 1.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..Default::default()
    });
}

const UPPER_LEFT_X: f32 = 3.0 * (PIXEL_SIZE + PITCH);
const UPPER_LEFT_Z: f32 = 3.0 * (PIXEL_SIZE + PITCH);

// O X X X
// X X X X
// X X X X
// X X X X
// Draw square with origin at top left
fn build_square_of_pixels_transforms(
    start_col: usize,
    start_row: usize,
    size: usize,
) -> Vec<Transform> {
    let mut pixel_transforms: Vec<Transform> = Vec::new();
    let max_col = start_col + size;
    let max_row = start_row + size;
    // Draw square
    for col in start_col..max_col {
        for row in start_row..max_row {
            let x = if col == 0 {
                0.0
            } else {
                (col as f32) * (PIXEL_SIZE + PITCH) - UPPER_LEFT_X
            };
            let z = if row == 0 {
                0.0
            } else {
                (row as f32) * (PIXEL_SIZE + PITCH) - UPPER_LEFT_Z
            };
            pixel_transforms.push(Transform::from_xyz(x, 0.0, z));
        }
    }
    pixel_transforms
}

fn build_horizontal_line_of_pixels_transforms(
    start_col: usize,
    start_row: usize,
    size: usize,
) -> Vec<Transform> {
    let mut pixel_transforms: Vec<Transform> = Vec::new();
    let max_col = start_col + size;
    for col in start_col..max_col {
        let x = if col == 0 {
            0.0
        } else {
            (col as f32) * (PIXEL_SIZE + PITCH) - UPPER_LEFT_X
        };
        let z = (start_row as f32) * (PIXEL_SIZE + PITCH) - UPPER_LEFT_Z;
        pixel_transforms.push(Transform::from_xyz(x, 0.0, z));
    }
    pixel_transforms
}

#[test]
fn what() {
    assert!(true);
    assert_eq!(4, 4);
}

fn setup_drawing(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pixel_mesh = meshes.add(Mesh::from(shape::Cube { size: PIXEL_SIZE }));
    let pixel_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        metallic: 0.5,
        perceptual_roughness: 0.5,
        ..Default::default()
    });

    let _parent_material = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        metallic: 0.1,
        perceptual_roughness: 0.1,
        ..Default::default()
    });

    // let parent_mesh_size = size as f32 * (PIXEL_SIZE + PITCH);
    // let parent_mesh = meshes.add(Mesh::from(shape::Cube {
    //     size: parent_mesh_size,
    // }));

    // let parent_x = (col as f32 * (PIXEL_SIZE + PITCH)) + (parent_mesh_size * 0.5);
    // let parent_y = (row as f32 * (PIXEL_SIZE + PITCH)) + (parent_mesh_size * 0.5);
    // let parent_transform = Transform::from_xyz(parent_x, -parent_mesh_size + PIXEL_SIZE, parent_y);
    // commands.spawn_bundle(PbrBundle {
    //     mesh: parent_mesh,
    //     material: parent_material,
    //     transform: parent_transform,
    //     ..Default::default()
    // });
    let horizontal_line_transforms = build_horizontal_line_of_pixels_transforms(0, 10, 80);
    for transform in horizontal_line_transforms {
        commands.spawn_bundle(PbrBundle {
            mesh: pixel_mesh.clone(),
            material: pixel_material.clone(),
            transform,
            ..Default::default()
        });
    }

    let col = 1;
    let row = 1;
    let size = 4;
    let pixel_transforms = build_square_of_pixels_transforms(col, row, size);
    for transform in pixel_transforms {
        commands.spawn_bundle(PbrBundle {
            mesh: pixel_mesh.clone(),
            material: pixel_material.clone(),
            transform,
            ..Default::default()
        });
    }
}

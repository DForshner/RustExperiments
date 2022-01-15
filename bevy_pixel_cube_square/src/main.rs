use bevy::prelude::*;

// Attempt at per pixel simulation by having pixel per entity.

const PIXEL_SIZE: f32 = 1.0;
const PITCH: f32 = 0.5;
const PIXEL_MAX_X_RES: usize = 360;
const PIXEL_MAX_Y_RES: usize = 180;

// TODO: Derive this from the camera height?
const UPPER_LEFT_X_CORRECTION: f32 = (PIXEL_MAX_X_RES - 1) as f32 * 0.5 * (PIXEL_SIZE + PITCH);
const UPPER_LEFT_Z_CORRECTION: f32 = (PIXEL_MAX_Y_RES - 1) as f32 * 0.5 * (PIXEL_SIZE + PITCH);

fn main() {
    App::new()
        //.insert_resource(Msaa { samples: 4 })
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
    // commands.spawn_bundle(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 5000000.0,
    //         range: 5000.0,
    //         radius: 400.0,
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     transform: Transform::from_xyz(0.0, 400.0, 0.0),
    //     ..Default::default()
    // });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 375.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
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

// O X X X
// X X X X
// X X X X
// X X X X
// Draw square with origin at top left
fn build_square_of_pixels_transforms(
    pixel_x_start: usize,
    pixel_y_start: usize,
    size: usize,
) -> Vec<Transform> {
    let mut pixel_transforms: Vec<Transform> = Vec::new();
    let max_pixel_x = pixel_x_start + size;
    let max_pixel_y = pixel_y_start + size;
    // Draw square
    for pixel_x in pixel_x_start..max_pixel_x {
        for pixel_y in pixel_y_start..max_pixel_y {
            let x = (pixel_x as f32) * (PIXEL_SIZE + PITCH) - UPPER_LEFT_X_CORRECTION;
            let z = (pixel_y as f32) * (PIXEL_SIZE + PITCH) - UPPER_LEFT_Z_CORRECTION;
            pixel_transforms.push(Transform::from_xyz(x, 0.0, z));
        }
    }
    pixel_transforms
}

// O X X X
fn build_horizontal_line_of_pixels_transforms(
    pixel_x_start: usize,
    pixel_y_start: usize,
    length: usize,
) -> Vec<Transform> {
    let mut pixel_transforms: Vec<Transform> = Vec::new();
    let max_pixel_x = pixel_x_start + length;
    for pixel_x in pixel_x_start..max_pixel_x {
        let x = (pixel_x as f32) * (PIXEL_SIZE + PITCH) - UPPER_LEFT_X_CORRECTION;
        let z = (pixel_y_start as f32) * (PIXEL_SIZE + PITCH) - UPPER_LEFT_Z_CORRECTION;
        pixel_transforms.push(Transform::from_xyz(x, 0.0, z));
    }
    pixel_transforms
}

// O
// X
// X
// X
fn build_vertical_line_of_pixels_transforms(
    pixel_x_start: usize,
    pixel_y_start: usize,
    length: usize,
) -> Vec<Transform> {
    let mut pixel_transforms: Vec<Transform> = Vec::new();
    let max_pixel_y = pixel_y_start + length;
    for pixel_y in pixel_y_start..max_pixel_y {
        let x = (pixel_x_start as f32) * (PIXEL_SIZE + PITCH) - UPPER_LEFT_X_CORRECTION;
        let z = (pixel_y as f32) * (PIXEL_SIZE + PITCH) - UPPER_LEFT_Z_CORRECTION;
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

    // let _parent_material = materials.add(StandardMaterial {
    //     base_color: Color::GRAY,
    //     metallic: 0.1,
    //     perceptual_roughness: 0.1,
    //     ..Default::default()
    // });

    let top_horizontal_line_transforms =
        build_horizontal_line_of_pixels_transforms(0, 0, PIXEL_MAX_X_RES);
    for transform in top_horizontal_line_transforms {
        commands.spawn_bundle(PbrBundle {
            mesh: pixel_mesh.clone(),
            material: pixel_material.clone(),
            transform,
            ..Default::default()
        });
    }

    let bottom_horizontal_line_transforms =
        build_horizontal_line_of_pixels_transforms(0, PIXEL_MAX_Y_RES, PIXEL_MAX_X_RES);
    for transform in bottom_horizontal_line_transforms {
        commands.spawn_bundle(PbrBundle {
            mesh: pixel_mesh.clone(),
            material: pixel_material.clone(),
            transform,
            ..Default::default()
        });
    }

    let left_vertical_line_transforms =
        build_vertical_line_of_pixels_transforms(0, 0, PIXEL_MAX_Y_RES);
    for transform in left_vertical_line_transforms {
        commands.spawn_bundle(PbrBundle {
            mesh: pixel_mesh.clone(),
            material: pixel_material.clone(),
            transform,
            ..Default::default()
        });
    }

    let right_vertical_line_transforms =
        build_vertical_line_of_pixels_transforms(PIXEL_MAX_X_RES, 0, PIXEL_MAX_Y_RES);
    for transform in right_vertical_line_transforms {
        commands.spawn_bundle(PbrBundle {
            mesh: pixel_mesh.clone(),
            material: pixel_material.clone(),
            transform,
            ..Default::default()
        });
    }

    let pixel_transforms = build_square_of_pixels_transforms(20, 20, 4);
    for transform in pixel_transforms {
        commands.spawn_bundle(PbrBundle {
            mesh: pixel_mesh.clone(),
            material: pixel_material.clone(),
            transform,
            ..Default::default()
        });
    }
}

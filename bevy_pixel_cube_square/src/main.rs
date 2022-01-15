use bevy::prelude::*;

const PIXEL_SIZE: f32 = 1.0;
const PITCH: f32 = 0.2;

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
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 20.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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
fn draw_square(origin_x: usize, origin_y: usize, size: usize) -> Vec<Transform> {
    let mut transforms: Vec<Transform> = Vec::new();
    let max_x = origin_x + size;
    let max_y = origin_y + size;
    for x in origin_x..max_x {
        for y in origin_y..max_y {
            let x_final = if x > 0 {
                x as f32 * (PIXEL_SIZE + PITCH)
            } else {
                x as f32
            };
            let y_final = if y > 0 {
                y as f32 * (PIXEL_SIZE + PITCH)
            } else {
                y as f32
            };
            transforms.push(Transform::from_xyz(x_final, 0.0, y_final));
        }
    }
    transforms
}

fn setup_drawing(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pixel_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        metallic: 0.5,
        perceptual_roughness: 0.5,
        ..Default::default()
    });

    // Draw
    let vec = draw_square(0, 0, 3);
    for t in vec {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: PIXEL_SIZE })),
            material: pixel_material.clone(),
            transform: t,
            ..Default::default()
        });
    }
}

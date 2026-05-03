use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 1. Spawn the core 2D Camera component directly
    commands.spawn(Camera2d);

    // 2. Spawn the Mesh2d and MeshMaterial2d as a tuple. 
    // Transform and Visibility are handled automatically via Required Components.
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, 100.0),     // Top vertex
            Vec2::new(-100.0, -100.0), // Bottom left vertex
            Vec2::new(100.0, -100.0),  // Bottom right vertex
        ))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.2, 0.2))), // Updated to explicit srgb
        Transform::from_translation(Vec3::ZERO), // Optional: explicitly set position
    ));
}

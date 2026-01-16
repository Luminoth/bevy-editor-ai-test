#![deny(warnings)]

use bevy::prelude::*;
use bevy_editor_ai_test::editor;

fn main() {
    App::new()
        // We use .set() here to configure the LogPlugin before it builds.
        // Bevy's LogPlugin initializes the global `tracing` subscriber immediately.
        // The `tracing` crate does not support replacing the global subscriber once initialized,
        // so we cannot "hook into" it later from the EditorPlugin.
        .add_plugins(DefaultPlugins.set(editor::log::log_plugin()))

        .add_plugins(editor::EditorPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}


fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Editor Camera is now spawned by EditorPlugin
}

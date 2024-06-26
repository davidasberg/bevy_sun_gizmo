use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_sun_gizmo::SunGizmoPlugin;

/// This example shows how to use the plugin gizmo plugin
/// with the pan orbit camera
/// Hold RightControl + L to activate the gizmo and controller.
/// Use left mouse button to rotate the camera
/// Use right mouse button to pan the camera around the gizmo
/// Use mouse wheel to zoom in and out
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SunGizmoPlugin::default(), // Add the plugin
            PanOrbitCameraPlugin,
        ))
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // Use a pan orbit camera to illustrate how the gizmo works when the camera moves
        PanOrbitCamera::default(),
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    let plane = meshes.add(Plane3d::default().mesh().size(10.0, 10.0));
    let cube = meshes.add(Cuboid::default().mesh());
    let sphere = meshes.add(Sphere::default().mesh());
    let green_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.4, 0.6, 0.3),
        ..default()
    });
    let gray_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.7, 0.7, 0.7),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: plane,
        material: green_material,
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: cube,
        material: gray_material.clone(),
        transform: Transform::from_xyz(2.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: sphere,
        material: gray_material,
        transform: Transform::from_xyz(-2.0, 0.5, 0.0),
        ..default()
    });
}

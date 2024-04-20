#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::{input::mouse::MouseMotion, prelude::*, transform::TransformSystem};
use std::f32::consts::{FRAC_PI_2, PI, TAU};

/// A plugin that adds a controller and gizmo for controlling directional lights
pub struct SunGizmoPlugin {
    /// Position of the gizmo in screen space. [0.0, 0.0] is top left, [1.0, 1.0] is bottom right
    pub position: Vec2,
    /// Size of the gizmo on screen
    pub size: f32,
    /// Time in seconds the gizmo persists after being activated by the user
    pub persist_time: f32,
    /// The combination of keys that will activate the gizmo
    pub key_bindings: Vec<KeyCode>,
    /// The sensitivity of the mouse controller
    pub sensitivity: f32,
    /// Line width of the gizmo
    pub line_width: f32,
}

impl Default for SunGizmoPlugin {
    fn default() -> Self {
        Self {
            position: Vec2::new(0.7, 0.7),
            size: 0.1,
            persist_time: 5.0,
            key_bindings: vec![KeyCode::ControlRight, KeyCode::KeyL],
            sensitivity: 0.5,
            line_width: 4.0,
        }
    }
}

impl Plugin for SunGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_sun)
            .add_systems(
                PostUpdate,
                draw_sun_gizmo.after(TransformSystem::TransformPropagate),
            )
            .add_event::<SunUpdatedEvent>()
            .insert_resource(SunGizmoControlConfig {
                key_bindings: self.key_bindings.clone(),
                sensitivity: self.sensitivity,
            })
            .insert_gizmo_group(
                SunGizmoGroup {
                    pos: Vec2::new(0.7, 0.7),
                    size: 0.1,
                    persist_time: 5.0,
                },
                GizmoConfig {
                    line_width: 4.0,
                    depth_bias: -1.0,
                    ..default()
                },
            );
    }
}

#[derive(Resource)]
struct SunGizmoControlConfig {
    pub key_bindings: Vec<KeyCode>,
    pub sensitivity: f32,
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct SunGizmoGroup {
    pub pos: Vec2,
    pub size: f32,
    pub persist_time: f32,
}

#[derive(Event)]
struct SunUpdatedEvent;

fn update_sun(
    mut query: Query<&mut Transform, With<DirectionalLight>>,
    mut mouse: EventReader<MouseMotion>,
    mut event_writer: EventWriter<SunUpdatedEvent>,
    input: Res<ButtonInput<KeyCode>>,
    config: Res<SunGizmoControlConfig>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };

    if input.all_pressed(config.key_bindings.clone()) {
        let mut delta = Vec2::ZERO;
        for event in mouse.read() {
            delta += event.delta * config.sensitivity;
        }

        let target: Vec3 = transform.up().into();
        let right: Vec3 = transform.right().into();
        let forward: Vec3 = transform.forward().into();
        let angle_between = forward.angle_between(target);
        transform.rotate_axis(right, angle_between.min(-delta.y * time.delta_seconds()));
        transform.rotate_axis(Vec3::Y, delta.x * time.delta_seconds());

        event_writer.send(SunUpdatedEvent);
    }
}

fn draw_sun_gizmo(
    directional_light: Query<&mut Transform, With<DirectionalLight>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut gizmos: Gizmos<SunGizmoGroup>,
    mut event_reader: EventReader<SunUpdatedEvent>,
    mut persist_time: Local<f32>,
    time: Res<Time>,
) {
    if !event_reader.is_empty() {
        *persist_time = gizmos.config_ext.persist_time;
        event_reader.clear();
    }

    if *persist_time < 0.0 {
        return;
    }

    *persist_time -= time.delta_seconds();

    let Ok(directional_light) = directional_light.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera.get_single() else {
        return;
    };
    let Some(viewport_size) = camera.logical_viewport_size() else {
        return;
    };

    let viewport_pos = viewport_size * gizmos.config_ext.pos;

    let Some(ray) = camera.viewport_to_world(camera_transform, viewport_pos) else {
        return;
    };

    let size = gizmos.config_ext.size;
    let origin = ray.get_point(1.0 / size);
    let x_color = Color::rgba(1.0, 0.4, 0.4, 1.0);
    let y_color = Color::rgba(0.4, 1.0, 0.4, 1.0);
    let z_color = Color::rgba(0.4, 0.4, 1.0, 1.0);

    // XZ Arc
    let y_forward_roation = Quat::IDENTITY;
    gizmos.arc_3d(TAU, 1.0, origin, y_forward_roation, y_color);
    gizmos.arc_3d(TAU, 0.8, origin, y_forward_roation, y_color.with_a(0.4));
    gizmos.arc_3d(TAU, 0.6, origin, y_forward_roation, y_color.with_a(0.25));
    gizmos.arc_3d(TAU, 0.4, origin, y_forward_roation, y_color.with_a(0.1));

    // YZ Arc
    let x_forward_rotation =
        Quat::from_rotation_arc(Vec3::Y, Vec3::X) * Quat::from_rotation_y(FRAC_PI_2);
    gizmos.arc_3d(PI, 1.0, origin, x_forward_rotation, x_color);
    // XY Arc
    let z_forward_rotation = Quat::from_rotation_arc(Vec3::Y, Vec3::Z);
    gizmos.arc_3d(PI, 1.0, origin, z_forward_rotation, z_color);

    gizmos.arrow(origin, origin + Vec3::X, x_color);
    gizmos.arrow(origin, origin + Vec3::Y, y_color);
    gizmos.arrow(origin, origin + Vec3::Z, z_color);

    let light_dir = *directional_light.forward();
    let start = origin - light_dir * 1.2;
    let end = origin - light_dir * 0.2;
    gizmos.arrow(start, end, Color::YELLOW);

    let projected_start = Vec3::new(start.x, origin.y, start.z);
    let x_axis_proj = Vec3::new(start.x, origin.y, origin.z);
    let z_axis_proj = Vec3::new(origin.x, origin.y, start.z);
    gizmos.line(start, projected_start, y_color);
    gizmos.line(projected_start, x_axis_proj, z_color);
    gizmos.line(projected_start, z_axis_proj, x_color);
}

use bevy::prelude::*;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (camera_orbit, camera_pan, camera_zoom));
    }
}

#[derive(Component)]
pub struct EditorCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for EditorCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            radius: 20.0,
            pitch: 45.0_f32.to_radians(),
            yaw: 45.0_f32.to_radians(),
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let camera = EditorCamera::default();
    let transform = calculate_camera_transform(&camera);

    commands.spawn((
        Camera3d::default(),
        transform,
        camera,
    ));
}

fn calculate_camera_transform(camera: &EditorCamera) -> Transform {
    let rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, -camera.pitch, 0.0);
    let position = camera.focus + rotation * Vec3::new(0.0, 0.0, camera.radius);

    Transform::from_translation(position).looking_at(camera.focus, Vec3::Y)
}

fn camera_orbit(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut EditorCamera, &mut Transform)>,
) {
    if !mouse_button.pressed(MouseButton::Right) {
        return;
    }

    let delta = mouse_motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    for (mut camera, mut transform) in query.iter_mut() {
        camera.yaw -= delta.x * 0.005;
        camera.pitch = (camera.pitch + delta.y * 0.005).clamp(0.1, std::f32::consts::FRAC_PI_2 - 0.1);

        *transform = calculate_camera_transform(&camera);
    }
}

fn camera_pan(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut EditorCamera, &mut Transform)>,
) {
    if !mouse_button.pressed(MouseButton::Middle) {
        return;
    }

    let delta = mouse_motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    for (mut camera, mut transform) in query.iter_mut() {
        let right = transform.right();
        let forward = transform.forward().reject_from(Vec3::Y).normalize_or_zero();

        let pan_speed = camera.radius * 0.002;
        camera.focus -= right * delta.x * pan_speed;
        camera.focus += forward * delta.y * pan_speed;

        *transform = calculate_camera_transform(&camera);
    }
}

fn camera_zoom(
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut query: Query<(&mut EditorCamera, &mut Transform)>,
) {
    let scroll = mouse_scroll.delta.y;
    if scroll == 0.0 {
        return;
    }

    for (mut camera, mut transform) in query.iter_mut() {
        camera.radius = (camera.radius - scroll * camera.radius * 0.1).clamp(5.0, 100.0);
        *transform = calculate_camera_transform(&camera);
    }
}

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod prelude {
    pub use crate::*;
}

/// Mouse sensitivity and movement speed
#[derive(Component)]
pub struct FpsCamera {
    pub sensitivity: f32,
    pub speed: Mat4,
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
}

impl Default for FpsCamera {
    fn default() -> Self {
        Self {
            sensitivity: 30.0,
            speed: Mat4::from_scale(Vec3::splat(1.0)),
            move_forward: KeyCode::W,
            move_backward: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,
        }
    }
}

/// Spawns the `Camera3dBundle` to be controlled
fn setup_player(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FpsCamera {
            ..Default::default()
        },
    ));
}

/// Handles keyboard input and movement
fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,

    mut query: Query<(&FpsCamera, &mut Transform)>, //    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if primary_window.get_single().is_err() {
        warn!("Primary window not found for `player_move`!");
        return;
    }
    for (setting, mut transform) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;

        let forward = -transform.local_z();
        let right = -transform.local_x();

        for key in keys.get_pressed() {
            let key = *key;
            if key == setting.move_forward {
                velocity += forward;
            } else if key == setting.move_backward {
                velocity -= forward;
            } else if key == setting.move_left {
                velocity -= right;
            } else if key == setting.move_right {
                velocity += right;
            }
        }
        velocity = velocity.normalize_or_zero();

        transform.translation += setting
            .speed
            .transform_vector3(velocity.normalize_or_zero())
            * time.delta_seconds();
    }
}

/// Handles looking around if cursor is locked
fn player_look(
    primary_window: Query<&Window, With<PrimaryWindow>>,

    mut motion: EventReader<MouseMotion>,
    mut query: Query<(&FpsCamera, &mut Transform)>,
) {
    let Ok(window) = primary_window.get_single() else {
          warn!("Primary window not found for `player_look`!");
          return;
    };

    let delta: Vec2 = motion.into_iter().map(|m| m.delta).sum();

    for (setting, mut transform) in query.iter_mut() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        let scale_factory = setting.sensitivity / window.height().min(window.width());
        pitch -= (delta.y * scale_factory).to_radians();
        yaw -= (delta.x * scale_factory).to_radians();

        pitch = pitch.clamp(-1.54, 1.54);

        // Order is important to prevent unintended roll
        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}

/// Contains everything needed to add first-person fly camera behavior to your game
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, player_move)
            .add_systems(Update, player_look);
    }
}

/// Same as [`PlayerPlugin`] but does not spawn a camera
pub struct NoCameraPlayerPlugin;
impl Plugin for NoCameraPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_move)
            .add_systems(Update, player_look);
    }
}

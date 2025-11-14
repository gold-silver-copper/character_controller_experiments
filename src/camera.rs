use std::f32::consts::TAU;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_enhanced_input::prelude::*;

use crate::{Player, user_input::Rotate};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (capture_cursor, sync_camera))
        .add_observer(rotate_camera);
}

fn sync_camera(
    mut camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
    player: Single<&GlobalTransform, With<Player>>,
) {
    camera.translation = player.translation();
}

fn rotate_camera(
    rotate: On<Fire<Rotate>>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let (mut yaw, mut pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

    let delta = rotate.value;
    yaw += delta.x.to_radians();
    pitch += delta.y.to_radians();
    pitch = pitch.clamp(-TAU / 4.0 + 0.01, TAU / 4.0 - 0.01);

    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

fn capture_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_transform_interpolation::TranslationEasingState;

use crate::character_controller::CharacterController;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, update_debug_text);
}

fn setup(mut commands: Commands) {
    commands.spawn((Node::default(), Text::default(), DebugText));
}

fn update_debug_text(
    mut text: Single<&mut Text, With<DebugText>>,
    kcc: Single<(&LinearVelocity, &TranslationEasingState), With<CharacterController>>,
) {
    let (velocity, interpolation) = kcc.into_inner();
    let velocity = velocity.0;
    let speed = velocity.length();
    let diff = interpolation.end.unwrap_or_default() - interpolation.start.unwrap_or_default();
    text.0 =
        format!("Velocity: {speed:.3} {velocity}\nInterpolation: {interpolation:#?}\nDiff: {diff}");
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct DebugText;

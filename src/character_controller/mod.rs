use avian3d::prelude::PhysicsSystems;
use bevy::prelude::*;

mod fixed_update_util;
mod input;
mod movement;

pub(crate) use input::{AccumulatedInput, Jump, Movement};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((fixed_update_util::plugin, input::plugin, movement::plugin))
        .configure_sets(
            FixedPostUpdate,
            CharacterControllerSystems::ApplyMovement.in_set(PhysicsSystems::First),
        );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(AccumulatedInput)]
pub(crate) struct CharacterController;

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum CharacterControllerSystems {
    ApplyMovement,
}

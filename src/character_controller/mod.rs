use avian3d::prelude::*;
use bevy::prelude::*;

mod fixed_update_util;
mod input;
#[allow(dead_code)]
mod quake_1;
#[allow(dead_code)]
mod quake_3;

pub(crate) use kcc::*;
use quake_1 as kcc;

pub(crate) use input::{Jump, Movement};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((fixed_update_util::plugin, input::plugin, kcc::plugin))
        .configure_sets(
            FixedPostUpdate,
            CharacterControllerSystems::ApplyMovement.in_set(PhysicsSystems::First),
        );
}

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum CharacterControllerSystems {
    ApplyMovement,
}

#[derive(Component)]
#[relationship(relationship_target = CharacterControllerForward)]
pub(crate) struct CharacterControllerForwardOf(pub(crate) Entity);

#[derive(Component)]
#[relationship_target(relationship = CharacterControllerForwardOf)]
pub(crate) struct CharacterControllerForward(Entity);

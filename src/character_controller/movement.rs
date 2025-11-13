use bevy::prelude::*;

use crate::character_controller::{CharacterControllerSystems, input::AccumulatedInput};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedPostUpdate,
        print_input.in_set(CharacterControllerSystems::ApplyMovement),
    );
}

fn print_input(input: Query<&AccumulatedInput>) {
    for input in input.iter() {
        println!("Input: {:?}", input);
    }
}

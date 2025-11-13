use bevy::prelude::*;

use crate::character_controller::{
    CharacterController, CharacterControllerState, CharacterControllerSystems,
    input::AccumulatedInput,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedPostUpdate,
        (run_kcc, print_input).in_set(CharacterControllerSystems::ApplyMovement),
    );
}

fn print_input(input: Query<&AccumulatedInput>) {
    for input in input.iter() {
        println!("Input: {:?}", input);
    }
}

fn run_kcc(
    world: &mut World,
    mut kccs: Local<
        QueryState<(
            Entity,
            &CharacterController,
            &CharacterControllerState,
            &AccumulatedInput,
        )>,
    >,
    mut input_scratch: Local<Vec<KccInput>>,
) {
    let dt = world.resource::<Time>().delta_secs();
    input_scratch.extend(
        kccs.iter(world)
            .map(|(entity, cfg, state, input)| KccInput {
                entity,
                cfg: *cfg,
                state: *state,
                input: *input,
                dt,
            }),
    );
    for input in input_scratch.drain(..) {
        let state: CharacterControllerState = match world.run_system_cached_with(air_move, input) {
            Ok(state) => state,
            Err(err) => {
                error!("Error running air_move system: {}", err);
                continue;
            }
        };
        *world
            .entity_mut(input.entity)
            .get_mut::<CharacterControllerState>()
            .unwrap() = state;
    }
}

#[derive(Debug, Clone, Copy)]
struct KccInput {
    entity: Entity,
    cfg: CharacterController,
    state: CharacterControllerState,
    input: AccumulatedInput,
    dt: f32,
}

fn air_move(
    In(KccInput {
        entity,
        cfg,
        mut state,
        input,
        dt,
    }): In<KccInput>,
    world: &mut World,
) -> Result<CharacterControllerState> {
    let dt = world.resource::<Time>().delta_secs();
    let movement = input.last_movement.unwrap_or_default();
    let cfg_speed = cfg.speed.normalize_or_zero();
    let mut wish_vel =
        cfg_speed.y * movement.y * Vec3::NEG_Z + cfg_speed.x * movement.x * Vec3::NEG_X;
    let (wish_dir, mut wish_speed) = Dir3::new_and_length(wish_vel).unwrap_or((Dir3::NEG_Z, 0.0));
    if wish_speed > cfg.max_speed {
        wish_vel *= cfg.max_speed / wish_speed;
        wish_speed = cfg.max_speed;
    }

    if state.grounded.is_some() {
        state.velocity.z = 0.0;
        accelerate();
        state.velocity.z -= cfg.gravity * dt;
        ground_move();
    } else {
        air_accelerate();
        state.velocity.z -= cfg.gravity * dt;
        fly_move()
    }
    Ok(state)
}

fn accelerate() {
    todo!();
}

fn ground_move() {
    todo!();
}

fn air_accelerate() {
    todo!();
}

fn fly_move() {
    todo!();
}

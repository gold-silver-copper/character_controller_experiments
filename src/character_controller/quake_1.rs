use avian3d::prelude::{ColliderAabb, LinearVelocity, RigidBodyColliders};
use bevy::{math::bounding::Aabb3d, prelude::*};

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
            &LinearVelocity,
            &GlobalTransform,
            &RigidBodyColliders,
        )>,
    >,
    mut colliders: Local<QueryState<&ColliderAabb>>,
    mut scratch: Local<Vec<(Vec3, Ctx)>>,
) {
    let dt = world.resource::<Time>().delta_secs();
    scratch.extend(kccs.iter(world).map(
        |(entity, cfg, state, input, vel, transform, collider_relations)| {
            (
                vel.0,
                Ctx {
                    entity,
                    cfg: *cfg,
                    state: *state,
                    input: *input,
                    dt,
                    origin: transform.translation(),
                    aabb: colliders
                        .iter_many(world, collider_relations.iter())
                        .fold(ColliderAabb::INVALID, |acc, aabb| acc.merged(*aabb)),
                },
            )
        },
    ));
    for (velocity, input) in scratch.drain(..) {
        let velocity: Vec3 = match world.run_system_cached_with(air_move, (velocity, input)) {
            Ok(state) => state,
            Err(err) => {
                error!("Error running air_move system: {}", err);
                continue;
            }
        };
        **world
            .entity_mut(input.entity)
            .get_mut::<LinearVelocity>()
            .unwrap() = velocity;
    }
}

#[derive(Debug, Clone, Copy)]
struct Ctx {
    entity: Entity,
    cfg: CharacterController,
    state: CharacterControllerState,
    input: AccumulatedInput,
    origin: Vec3,
    aabb: ColliderAabb,
    dt: f32,
}

fn air_move(In((mut velocity, ctx)): In<(Vec3, Ctx)>) -> Result<Vec3> {
    let movement = ctx.input.last_movement.unwrap_or_default();
    let cfg_speed = ctx.cfg.speed.normalize_or_zero();
    let mut wish_vel =
        cfg_speed.y * movement.y * Vec3::NEG_Z + cfg_speed.x * movement.x * Vec3::NEG_X;
    let (wish_dir, mut wish_speed) = Dir3::new_and_length(wish_vel).unwrap_or((Dir3::NEG_Z, 0.0));
    if wish_speed > ctx.cfg.max_speed {
        wish_vel *= ctx.cfg.max_speed / wish_speed;
        wish_speed = ctx.cfg.max_speed;
    }

    if ctx.state.grounded.is_some() {
        velocity.z = 0.0;
        velocity = accelerate(wish_dir, wish_speed, velocity, ctx);
        velocity.z -= ctx.cfg.gravity * ctx.dt;
        ground_move();
    } else {
        velocity = air_accelerate(wish_dir, wish_speed, velocity, ctx);
        velocity.z -= ctx.cfg.gravity * ctx.dt;
        fly_move()
    }
    Ok(velocity)
}

fn accelerate(wish_dir: Dir3, wish_speed: f32, velocity: Vec3, ctx: Ctx) -> Vec3 {
    let current_speed = velocity.dot(wish_dir.into());
    // right here is where air strafing happens: `current_speed` is close to 0 when we want to move perpendicular to
    // our current velocity, making `add_speed` large.
    let add_speed = wish_speed - current_speed;
    if add_speed <= 0.0 {
        return velocity;
    }

    let accel_speed = f32::min(ctx.cfg.acceleration * ctx.dt * wish_speed, add_speed);
    velocity + accel_speed * wish_dir
}

fn air_accelerate(wish_dir: Dir3, wish_speed: f32, velocity: Vec3, ctx: Ctx) -> Vec3 {
    let wish_speed = f32::min(wish_speed, ctx.cfg.air_speed);
    accelerate(wish_dir, wish_speed, velocity, ctx)
}

fn ground_move() {
    todo!();
}

fn fly_move() {
    todo!();
}

fn friction(velocity: Vec3, ctx: Ctx) -> Vec3 {
    let speed = velocity.length();
    if speed < 0.025 {
        return Vec3::ZERO;
    }

    let mut friction_hz = ctx.cfg.friction_hz;
    // if the leading edge is over a dropoff, increase friction
    if ctx.state.grounded.is_some() {
        let mut start = ctx.origin + velocity / speed * 0.4;
        start.z = ctx.origin.z + ctx.aabb.min.z;
        let mut stop = start;
        stop.z = start.z - 0.85;

        /*
        let trace = todo!();

        if trace.fraction == 1 {
            friction *= 2
        }
         */
    }
    let drop = if ctx.state.grounded.is_some() {
        let stop_speed = f32::max(speed, ctx.cfg.stop_speed);
        stop_speed * friction_hz * ctx.dt
    } else {
        0.0
    };
    let new_speed = f32::max(speed - drop, 0.0);
    velocity / speed * new_speed
}

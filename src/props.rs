use avian3d::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::{physics::SceneCollidersReady, prelude::*};
use std::ops::Deref;

#[point_class(base(Transform, Visibility), model("models/cube.glb"))]
#[component(on_add = on_add_prop::<Self>)]
#[derive(Default, Deref)]
struct Cube {
    dynamic: bool,
}

#[point_class(base(Transform, Visibility), model("models/cone.glb"))]
#[component(on_add = on_add_prop::<Self>)]
#[derive(Default, Deref)]
struct Cone {
    dynamic: bool,
}

#[point_class(base(Transform, Visibility), model("models/cylinder.glb"))]
#[component(on_add = on_add_prop::<Self>)]
#[derive(Default, Deref)]
struct Cylinder {
    dynamic: bool,
}

#[point_class(base(Transform, Visibility), model("models/capsule.glb"))]
#[component(on_add = on_add_prop::<Self>)]
#[derive(Default, Deref)]
struct Capsule {
    dynamic: bool,
}

#[point_class(base(Transform, Visibility), model("models/sphere.glb"))]
#[component(on_add = on_add_prop::<Self>)]
#[derive(Default, Deref)]
struct Sphere {
    dynamic: bool,
}

fn on_add_prop<T: QuakeClass + Deref<Target = bool>>(mut world: DeferredWorld, ctx: HookContext) {
    if world.is_scene_world() {
        return;
    }
    let dynamic = *world.get::<T>(ctx.entity).unwrap().deref();
    let assets = world.resource::<AssetServer>().clone();
    world
        .commands()
        .entity(ctx.entity)
        .insert((
            SceneRoot(
                assets
                    .load(GltfAssetLabel::Scene(0).from_asset(T::CLASS_INFO.model_path().unwrap())),
            ),
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
            if dynamic {
                RigidBody::Dynamic
            } else {
                RigidBody::Static
            },
            TransformInterpolation,
        ))
        .observe(|ready: On<SceneCollidersReady>, mut commands: Commands| {
            for collider in &ready.collider_entities {
                commands.entity(*collider).insert(ColliderDensity(100.0));
            }
        });
}

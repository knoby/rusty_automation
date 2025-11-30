use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
#[require(
    ConveyorVelocity,
    Transform,
    ActiveHooks::MODIFY_SOLVER_CONTACTS,
    RigidBody::Fixed,
    super::Actuator
)]
pub struct Conveyor;

impl Conveyor {
    pub fn new(size: Vec3) -> (Conveyor, Collider) {
        (
            Self,
            Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
        )
    }
}

#[derive(Component, Default)]

pub struct ConveyorVelocity {
    pub velocity: f32,
}

#[derive(SystemParam)]
pub struct ConveyorHook<'w, 's> {
    pub query: Query<'w, 's, &'static ConveyorVelocity>,
}

impl BevyPhysicsHooks for ConveyorHook<'_, '_> {
    fn modify_solver_contacts(&self, context: ContactModificationContextView) {
        let collider1 = context.collider1();
        let collider2 = context.collider2();
        for solver_contact in &mut *context.raw.solver_contacts {
            if let Ok(velocity) = self.query.get(collider1).or(self.query.get(collider2)) {
                solver_contact.tangent_velocity.x = velocity.velocity;
            };
        }
    }
}

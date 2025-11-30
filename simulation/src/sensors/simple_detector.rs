use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
#[require(
    super::Sensor,
    RigidBody::Fixed,
    ActiveEvents::COLLISION_EVENTS,
    Transform,
    super::DigitalSensorValue
)]
pub struct SimpleDetector;

impl SimpleDetector {
    pub fn new(size: Vec3) -> (Self, Collider, Sensor) {
        (
            Self,
            Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
            Sensor,
        )
    }
}

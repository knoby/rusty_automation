use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::CollisionEventFlags};

use crate::mqtt::MqttSensorEvent;

pub mod simple_detector;

#[derive(Component, Default)]
pub struct Sensor;

#[derive(Component, Default)]
pub struct DigitalSensorValue {
    pub state: bool,
}

pub fn update_digital_sensors(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(&mut DigitalSensorValue, Option<&crate::mqtt::MqttTopic>)>,
    mut sensor_event: EventWriter<crate::mqtt::MqttSensorEvent>,
) {
    // Handle all colission events
    for collision_event in collision_events.read() {
        // Check Event if the collision does include at least one Entity Sensor Flag set
        let (state, entity) = match collision_event {
            CollisionEvent::Started(entity, entity1, collision_event_flags) => {
                if *collision_event_flags == CollisionEventFlags::SENSOR {
                    (true, Some([entity, entity1]))
                } else {
                    (true, None)
                }
            }
            CollisionEvent::Stopped(entity, entity1, collision_event_flags) => {
                if *collision_event_flags == CollisionEventFlags::SENSOR {
                    (false, Some([entity, entity1]))
                } else {
                    (false, None)
                }
            }
        };

        // If a sensor is included update the DigitalSensorValue component
        if let Some(entity) = entity {
            for entity in entity {
                if let Ok((mut sensor, bmk)) = query.get_mut(*entity)
                    && state != sensor.state
                {
                    // Update Sensor Value
                    sensor.state = state;
                    // If the Sensor has an bmk update the Value on the MQTT Server
                    if let Some(bmk) = bmk {
                        sensor_event.write(MqttSensorEvent {
                            bmk: bmk.name.clone(),
                            value: state,
                        });
                    }
                }
            }
        }
    }
}

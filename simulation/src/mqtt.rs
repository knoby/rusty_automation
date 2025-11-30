use bevy::prelude::*;
use bevy_mqtt::{
    MqttClientConnected, MqttClientError, MqttConnectError, MqttEvent, MqttPublishOutgoing,
    MqttSetting, SubscribeTopic, TopicMessage,
    rumqttc::{self, MqttOptions, QoS},
};

use crate::actuators::conveyor::{Conveyor, ConveyorVelocity};

#[derive(Component)]
pub struct MqttTopic {
    pub name: String,
}

pub fn setup_mqtt_client(mut commands: Commands) {
    commands.spawn(MqttSetting {
        mqtt_options: MqttOptions::new("machine_simulation", "localhost", 1883),
        cap: 10,
    });
}

pub fn handle_messages(mut mqtt_events: EventReader<MqttEvent>) {
    for event in mqtt_events.read() {
        if let rumqttc::Event::Incoming(rumqttc::Incoming::Publish(publish)) = &event.event {
            println!("Received on {}: {:?}", publish.topic, publish.payload);
        }
    }
}

pub fn handle_errors(
    mut connect_errors: EventReader<MqttConnectError>,
    mut client_errors: EventReader<MqttClientError>,
) {
    for error in connect_errors.read() {
        eprintln!("MQTT connection error: {:?}", error.error);
    }

    for error in client_errors.read() {
        eprintln!("MQTT client error: {:?}", error.error);
    }
}

pub fn publish_sensor_value(
    mqtt_clients: Query<Entity, With<MqttClientConnected>>,
    mut publish_topic: EventWriter<MqttPublishOutgoing>,
    mut sensor_events: EventReader<MqttSensorEvent>,
) {
    for event in sensor_events.read() {
        for client_entity in mqtt_clients.iter() {
            publish_topic.write(MqttPublishOutgoing {
                entity: client_entity,
                topic: format!("sensor/digital/{}", event.bmk),
                qos: QoS::AtLeastOnce,
                retain: false,
                payload: event.value.to_string().as_bytes().to_vec(),
            });
        }
    }
}

pub fn subscribe_conveyor_velocity(
    mut commands: Commands,
    mqtt_clients: Query<Entity, Added<MqttClientConnected>>,
    conveyor: Query<&MqttTopic, With<Conveyor>>,
) {
    for client_entity in mqtt_clients.iter() {
        for bmk in conveyor {
            // Subscribe using component-based approach with MQTT wildcards
            let topic_entity = commands
                .spawn(
                    SubscribeTopic::new(format!("actuator/motor/{}", bmk.name), QoS::AtMostOnce)
                        .unwrap(),
                )
                .observe(
                    |trigger: Trigger<TopicMessage>,
                     mut conveyor: Query<(&mut ConveyorVelocity, &MqttTopic)>| {
                        // Find Conveyor
                        let (mut conveyor_velocity, _) = conveyor
                            .iter_mut()
                            .find(|(_, bmk)| trigger.topic.ends_with(&bmk.name))
                            .unwrap();
                        let velocity = str::from_utf8(&trigger.payload).unwrap().parse().unwrap();
                        (conveyor_velocity.velocity) = velocity;
                    },
                )
                .id();

            // Link topic subscription to client
            commands.entity(client_entity).add_child(topic_entity);
        }
    }
}

#[derive(Event)]
pub struct MqttSensorEvent {
    pub bmk: String,
    pub value: bool,
}

use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_mqtt::MqttPlugin;
use bevy_rapier3d::prelude::*;

use crate::mqtt::MqttTopic;

mod actuators;
mod misc;
mod mqtt;
mod sensors;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<actuators::conveyor::ConveyorHook>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FlyCameraPlugin)
        .add_plugins(MqttPlugin)
        .add_event::<mqtt::MqttSensorEvent>()
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_scene)
        .add_systems(
            Startup,
            (
                mqtt::setup_mqtt_client,
                mqtt::handle_errors,
                mqtt::handle_messages,
            ),
        )
        .add_systems(Update, mqtt::subscribe_conveyor_velocity)
        .add_systems(Update, mqtt::publish_sensor_value)
        .add_systems(Update, sensors::update_digital_sensors)
        .run();
}

fn setup_graphics(mut commands: Commands, _assets: Res<AssetServer>) {
    // Add a camera so we can see the debug-render.
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .insert(FlyCamera {
            max_speed: 0.02,
            ..Default::default()
        });
}

fn setup_scene(mut commands: Commands) {
    const CONVEYOR_LENGTH: f32 = 1.0;
    const CONVEYOR_WIDTH: f32 = 0.4;
    const CONVEYOR_HEIGHT: f32 = 0.05;

    commands
        .spawn(actuators::conveyor::Conveyor::new(Vec3 {
            x: CONVEYOR_LENGTH,
            y: CONVEYOR_HEIGHT,
            z: CONVEYOR_WIDTH,
        }))
        .insert(MqttTopic {
            name: "M1".to_string(),
        });

    // Insert Cube
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.1, 0.1, 0.1))
        .insert(Sleeping::disabled())
        .insert(Transform::from_xyz(0.0, 1.0, 0.0));

    // Insert Sensors
    commands
        .spawn(sensors::simple_detector::SimpleDetector::new(vec3(
            0.02,
            0.02,
            CONVEYOR_WIDTH,
        )))
        .insert(Transform::from_xyz(-CONVEYOR_LENGTH / 2.0, 0.1, 0.0))
        .insert(MqttTopic {
            name: "B2".to_string(),
        });

    commands
        .spawn(sensors::simple_detector::SimpleDetector::new(vec3(
            0.02,
            0.02,
            CONVEYOR_WIDTH,
        )))
        .insert(Transform::from_xyz(CONVEYOR_LENGTH / 2.0, 0.1, 0.0))
        .insert(MqttTopic {
            name: "B1".to_string(),
        });
}

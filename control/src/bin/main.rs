use std::{sync::Arc, time::Duration};

use ethercrab::{
    MainDevice, MainDeviceConfig, PduStorage, Timeouts,
    std::{ethercat_now, tx_rx_task},
};
use tokio::time::MissedTickBehavior;

/// Maximum number of SubDevices that can be stored. This must be a power of 2 greater than 1.
const MAX_SUBDEVICES: usize = 16;
/// Maximum PDU data payload size - set this to the max PDI size or higher.
const MAX_PDU_DATA: usize = 1100;
/// Maximum number of EtherCAT frames that can be in flight at any one time.
const MAX_FRAMES: usize = 8;
/// Maximum total PDI length.
const PDI_LEN: usize = 64;

static PDU_STORAGE: PduStorage<MAX_FRAMES, MAX_PDU_DATA> = PduStorage::new();

#[tokio::main]
async fn main() {
    // Enable Logger
    pretty_env_logger::init();

    // Select interface
    let interface = std::env::args().nth(1).unwrap_or_else(|| "en9".to_string());
    log::info!("Using Interface '{}", interface);

    // Setup tx,rx and space
    let (tx, rx, pdu_loop) = PDU_STORAGE.try_split().expect("can only split once");

    // Setup the EtherCat Main Device
    let timouts = Timeouts {
        wait_loop_delay: Duration::from_millis(2),
        mailbox_response: Duration::from_millis(1000),
        ..Default::default()
    };
    let maindevice_config = MainDeviceConfig::default();
    let maindevice = Arc::new(MainDevice::new(pdu_loop, timouts, maindevice_config));

    // Spawn TXRX Task
    tokio::spawn(tx_rx_task(&interface, tx, rx).expect("spawn TX/RX task"));

    // Initialise all devices in a single group and wait for all devices to be in PRE_OP
    let group = maindevice
        .init_single_group::<MAX_SUBDEVICES, PDI_LEN>(ethercat_now)
        .await
        .expect("Init");

    // Display number of devices on the bus
    log::info!("Fund {} SubDevices on the bus", group.len());

    // Display information about the found devices
    for subdevice in group.iter(&maindevice) {
        log::info!(
            "-> SubDevice {:#06x} {}",
            subdevice.configured_address(),
            subdevice.name(),
        );
    }

    // Setupt tickrate for the loop
    let mut tick_interval = tokio::time::interval(Duration::from_millis(10));
    tick_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let group = group.into_op(&maindevice).await.expect("PreOp->SafeOp->Op");

    // Main action loop
    loop {
        group.tx_rx(&maindevice).await.expect("TX/RX");

        // Increment every output byte for every SubDevice by one
        for subdevice in group.iter(&maindevice) {
            let mut io = subdevice.io_raw_mut();

            for byte in io.outputs().iter_mut() {
                *byte = byte.wrapping_add(1);
            }
        }

        tick_interval.tick().await;
    }
}

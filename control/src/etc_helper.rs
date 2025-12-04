use std::sync::Arc;

use anyhow::Result;
use ethercrab::{MainDevice, MainDeviceConfig, PduLoop, PduRx, PduStorage, PduTx, Timeouts};
use tokio::sync::Mutex;

/// Maximum number of SubDevices that can be stored. This must be a power of 2 greater than 1.
const MAX_SUBDEVICES: usize = 64;
/// Maximum PDU data payload size - set this to the max PDI size or higher.
const MAX_PDU_DATA: usize = PduStorage::element_size(1100);
/// Maximum number of EtherCAT frames that can be in flight at any one time.
const MAX_FRAMES: usize = 16;
/// Maximum total PDI length.
const PDI_LEN: usize = 64;

static PDU_STORAGE: PduStorage<MAX_FRAMES, MAX_PDU_DATA> = PduStorage::new();
static TX_RX_PDULOOP: Mutex<Option<(PduTx, PduRx, PduLoop)>> = Mutex::const_new(None);

pub async fn bus_scan(interface: String) -> Result<Vec<String>> {
    log::info!("Discovering EtherCAT devices on {}...", interface);

    let (tx, rx, pdu_loop) = PDU_STORAGE
        .try_split()
        .unwrap_or_else(|_| TX_RX_PDULOOP.blocking_lock().take().unwrap());

    let maindevice = MainDevice::new(
        pdu_loop,
        Timeouts::default(),
        MainDeviceConfig {
            dc_static_sync_iterations: 0,
            ..MainDeviceConfig::default()
        },
    );

    let tx_rx_task = tokio::spawn(ethercrab::std::tx_rx_task(&interface, tx, rx).unwrap());
    {
        let group = maindevice
            .init_single_group::<MAX_SUBDEVICES, PDI_LEN>(ethercrab::std::ethercat_now)
            .await
            .expect("Init");

        println!("Discovered {} SubDevices", group.len());

        for subdevice in group.iter(&maindevice) {
            println!(
                "--> SubDevice {:#06x} name {}, description {:?}, {}",
                subdevice.configured_address(),
                subdevice.name(),
                subdevice
                    .description()
                    .await
                    .expect("Failed to read description"),
                subdevice.identity()
            );
        }
    }
    // Cancle RX/TX Task
    let pdu_loop = unsafe { maindevice.release_all() };
    let (tx, rx) = tx_rx_task.await.unwrap().unwrap();

    {
        let mut tx_rx_pduloop = TX_RX_PDULOOP.lock().await;
        tx_rx_pduloop.insert((tx, rx, pdu_loop));
    }

    log::info!("Done.");
    Ok(Vec::new())
}

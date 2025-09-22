use esp_idf_hal::{
    delay::Delay,
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};
use esp_idf_svc::{eventloop::EspSystemEventLoop, log::*, sys::*};
use util::nmea::{NMEAData, NMEASentenceType};

use crate::util::wifi;

use anyhow::{bail, Result};

mod util {
    pub mod nmea;
    pub mod sensors;
    pub mod wifi;
}

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

/**
 * - Initialize the sensor manager and the TCP client
 * - Send sensor data in NMEA format (via Wi-Fi) to the TCP server (XCSoar)
 */
fn main() -> Result<()> {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let app_config = CONFIG;

    log::info!(
        "Wi-Fi SSID and password: {} - {}",
        app_config.wifi_ssid,
        app_config.wifi_psk
    );

    let i2c_peripheral = peripherals.i2c0;
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let mut i2c = I2cDriver::new(i2c_peripheral, sda, scl, &config)?;
    let sysloop = EspSystemEventLoop::take()?;

    scan_devices(&mut i2c);

    let _wifi = match wifi::wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => {
            println!("Connected to Wi-Fi network!");
            inner
        }
        Err(err) => {
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    let nmeadata = NMEAData::new(
        NMEAData::build_position(43.0, 14.0, 1382.0),
        NMEAData::build_motion(2.0, 1.0, 452.0, 19.0, 0.0),
        NMEAData::build_environment(1013.25, 19.0, 0.0),
    );

    let pov = nmeadata.get_data_string(NMEASentenceType::Pov);
    let peya = nmeadata.get_data_string(NMEASentenceType::Peya);
    let peyi = nmeadata.get_data_string(NMEASentenceType::Peyi);

    log::info!("{}\n{}\n{}", pov, peya, peyi);

    let delay = Delay::new_default();

    loop {
        delay.delay_ms(1000);
    }

    //TODO: Initialize sensors using sensors.rs
    /* let mut sensor_manager = SensorManager::new()?;

    //let delay = Delay::new_default();

    match sensor_manager.init().is_ok() {
        true => log::info!("Sensors initialized successfully"),
        false => log::error!("Failed to initialize sensors"),
    }; */

    /* loop {
        //let (accel_gyro_data, barometer_data, gps_data) =
        let barometer_data = sensor_manager.get_measurements()?;

        //println!("accel_gyro_data: {:#?}", accel_gyro_data);
        println!("barometer_data: {:#?}", barometer_data);
        //println!("gps_data: {:#?}", gps_data);

        //TODO: Use TCP client stream here and send the NMEA data from sensors.rs
        log::info!("Data sent successfully");
        delay.delay_ms(1000);
    } */
}

fn scan_devices(i2c: &mut I2cDriver) {
    println!("Starting I2C scan...");
    for addr in 0x08..0x78 {
        // Write empty message to check device presence
        let result = i2c.write(addr, &[], 1000);
        if result.is_ok() {
            println!("Found device at address: 0x{:02X}", addr);
        }
    }
    println!("Scan complete!");
}

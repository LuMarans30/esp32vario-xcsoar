use esp_idf_hal::{
    //delay::Delay,
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};
use esp_idf_svc::{log::*, sys::*};
use util::{
    nmea::{NMEAData, NMEASentenceType},
    //sensors::SensorManager,
};

//use crate::util::tcp_client;

mod util {
    pub mod nmea;
    pub mod sensors;
    pub mod tcp_client;
}

/**
 * - Initialize the sensor manager and the TCP client
 * - Send sensor data in NMEA format (via Wi-Fi) to the TCP server (XCSoar)
 */
fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let mut i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    scan_devices(&mut i2c);

    let nmeadata = NMEAData::new(
        NMEAData::build_position(43.0, 14.0, 1382.0),
        NMEAData::build_motion(2.0, 1.0, 452.0, 19.0, 0.0),
        NMEAData::build_environment(1013.25, 19.0, 0.0),
    );

    let pov = nmeadata.get_data_string(NMEASentenceType::Pov);
    println!("{}", pov);

    let peya = nmeadata.get_data_string(NMEASentenceType::Peya);
    println!("{}", peya);

    let peyi = nmeadata.get_data_string(NMEASentenceType::Peyi);
    println!("{}", peyi);

    //TODO: Initialize TCP client using tcp_client.rs

    //FIXME: Fix Peripherals borrowing (Peripherals::take().unwrap()) since it's not possible
    //Until the borrowing is fixed, the TCP client cannot be initialized
    /* let _ = tcp_client::init().unwrap_or_else(|e| panic!("Failed to initialize TCP client: {}", e));
    (); */

    //TODO: Initialize sensors using sensors.rs
    /* let mut sensor_manager = SensorManager::new().unwrap();

    //let delay = Delay::new_default();

    match sensor_manager.init().is_ok() {
        true => log::info!("Sensors initialized successfully"),
        false => log::error!("Failed to initialize sensors"),
    }; */

    /* loop {
        //let (accel_gyro_data, barometer_data, gps_data) =
        let barometer_data = sensor_manager.get_measurements().unwrap();

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

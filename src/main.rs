use esp_idf_hal::{delay::Delay, prelude::*};
use esp_idf_svc::{log::*, sys::*};
use util::{nmea::NMEAData, sensors::SensorManager};

use crate::util::tcp_client;

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

    let delay = Delay::new_default();

    /*
    let nmeadata = NMEAData {
        latitude: 43.0,
        longitude: 14.0,
        altitude: 1382.0,
        speed: 2.0,
        heading: 1.0,
        static_pressure: 452.0,
        temperature: 19.0,
        average_cps: 0.0,
        acceleration_x: 2.0,
        acceleration_y: -1.5,
        acceleration_z: 0.3,
    };

    let pov = nmeadata.get_data_string("pov");
    println!("{}", pov);

    let peya = nmeadata.get_data_string("peya");
    println!("{}", peya);

    let peyi = nmeadata.get_data_string("peyi");
    println!("{}", peyi);
    */

    //TODO: Initialize TCP client using tcp_client.rs

    //FIXME: Fix Peripherals borrowing (Peripherals::take().unwrap()) since it's not possible
    //Until the borrowing is fixed, the TCP client cannot be initialized
    /* let _ = tcp_client::init().unwrap_or_else(|e| panic!("Failed to initialize TCP client: {}", e));
    (); */

    //TODO: Initialize sensors using sensors.rs
    let mut sensor_manager = SensorManager::new().unwrap();

    if sensor_manager.init().is_ok() {
        log::info!("Sensors initialized successfully");
    } else {
        log::error!("Failed to initialize sensors");
    }

    loop {
        sensor_manager.get_measurements().unwrap();
        //TODO: Use TCP client stream here and send the NMEA data from sensors.rs
        log::info!("Data sent successfully");
        delay.delay_ms(1000);
    }
}

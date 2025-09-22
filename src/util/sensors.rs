use esp_idf_hal::{delay::BLOCK, i2c::*, prelude::*, sys::EspError};
use std::sync::{Arc, Mutex};

mod accelerometer_gyro {
    pub mod mpu9250;
}

mod barometer {
    pub mod bmp280;
}

mod gps {
    pub mod atgm336h;
}

use accelerometer_gyro::mpu9250::{AccelGyro, AccelGyroData};
use barometer::bmp280::{Barometer, BarometerData};
use gps::atgm336h::{Gps, GpsData};

pub trait Sensor<'a> {
    type Data;

    fn new(i2c: Arc<Mutex<I2cDriver<'a>>>, address: u8) -> Self;
    fn init(&mut self) -> Result<(), EspError>;
    fn get_measurement(&mut self) -> Result<Self::Data, EspError>;
}

pub struct SensorManager<'a> {
    accel_gyro: AccelGyro<'a>,
    barometer: Barometer<'a>,
    gps: Gps<'a>,
}

impl<'a> SensorManager<'a> {
    pub fn new() -> Result<Self, EspError> {
        let peripherals = Peripherals::take().unwrap();
        let config = I2cConfig::new().baudrate(400.kHz().into());

        let i2c_driver = I2cDriver::new(
            peripherals.i2c0,
            peripherals.pins.gpio5,
            peripherals.pins.gpio6,
            &config,
        )?;

        let i2c = Arc::new(Mutex::new(i2c_driver));

        Ok(SensorManager {
            accel_gyro: AccelGyro::new(i2c.clone(), 0x68),
            barometer: Barometer::new(i2c.clone(), 0x76),
            gps: Gps::new(i2c, 0x42),
        })
    }

    pub fn init(&mut self) -> Result<(), EspError> {
        self.accel_gyro.init()?;
        self.barometer.init()?;
        self.gps.init()?;
        Ok(())
    }

    pub fn get_measurements(
        &mut self,
    ) -> Result<(AccelGyroData, BarometerData, GpsData), EspError> {
        Ok((
            self.accel_gyro.get_measurement()?,
            self.barometer.get_measurement()?,
            self.gps.get_measurement()?,
        ))
    }
}

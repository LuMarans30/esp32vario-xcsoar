use super::super::*;

#[derive(Debug, Clone)]
pub struct BarometerData {
    pub temperature: f32,
    pub pressure: f32,
    pub altitude: f32,
}

pub struct Barometer<'a> {
    address: u8,
    i2c: Arc<Mutex<I2cDriver<'a>>>,
}

impl<'a> Sensor<'a> for Barometer<'a> {
    type Data = BarometerData;

    fn new(i2c: Arc<Mutex<I2cDriver<'a>>>, address: u8) -> Self {
        Barometer { address, i2c }
    }

    fn init(&mut self) -> Result<(), EspError> {
        let mut i2c = self.i2c.lock().unwrap();
        // Configure the sensor: Normal mode, temperature and pressure oversampling x1
        i2c.write(self.address, &[0xF4, 0x27], BLOCK)?;
        // Configure the filter settings
        i2c.write(self.address, &[0xF5, 0x0C], BLOCK)?;
        Ok(())
    }

    fn get_measurement(&mut self) -> Result<Self::Data, EspError> {
        let mut buffer = [0u8; 6];
        let mut i2c = self.i2c.lock().unwrap();

        i2c.write_read(self.address, &[0xF7], &mut buffer, BLOCK)?;

        // Convert the raw values to actual measurements
        // Note: These conversions are simplified. Real implementation would need calibration data
        let pressure = (u32::from(buffer[0]) << 12
            | u32::from(buffer[1]) << 4
            | u32::from(buffer[2]) >> 4) as f32
            / 100.0;

        let temperature = (u32::from(buffer[3]) << 12
            | u32::from(buffer[4]) << 4
            | u32::from(buffer[5]) >> 4) as f32
            / 100.0;

        // Simplified altitude calculation
        let altitude = 44330.0 * (1.0 - (pressure / 1013.25).powf(1.0 / 5.255));

        Ok(BarometerData {
            temperature,
            pressure,
            altitude,
        })
    }
}

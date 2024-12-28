use super::super::*;

#[derive(Debug, Clone)]
pub struct AccelGyroData {
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
}

pub struct AccelGyro<'a> {
    address: u8,
    i2c: Arc<Mutex<I2cDriver<'a>>>,
}

impl<'a> Sensor<'a> for AccelGyro<'a> {
    type Data = AccelGyroData;

    fn new(i2c: Arc<Mutex<I2cDriver<'a>>>, address: u8) -> Self {
        AccelGyro { address, i2c }
    }

    fn init(&mut self) -> Result<(), EspError> {
        let mut i2c = self.i2c.lock().unwrap();
        // Power management register 1
        i2c.write(self.address, &[0x6B, 0x00], BLOCK)?;
        // Set gyro full scale range to ±250°/s
        i2c.write(self.address, &[0x1B, 0x00], BLOCK)?;
        // Set accelerometer full scale range to ±2g
        i2c.write(self.address, &[0x1C, 0x00], BLOCK)?;
        Ok(())
    }

    fn get_measurement(&mut self) -> Result<Self::Data, EspError> {
        let mut buffer = [0u8; 14];
        let mut i2c = self.i2c.lock().unwrap();

        i2c.write_read(self.address, &[0x3B], &mut buffer, BLOCK)?;

        // Convert the raw values to actual measurements
        let accel_x = (i16::from(buffer[0]) << 8 | i16::from(buffer[1])) as f32 / 16384.0;
        let accel_y = (i16::from(buffer[2]) << 8 | i16::from(buffer[3])) as f32 / 16384.0;
        let accel_z = (i16::from(buffer[4]) << 8 | i16::from(buffer[5])) as f32 / 16384.0;

        let gyro_x = (i16::from(buffer[8]) << 8 | i16::from(buffer[9])) as f32 / 131.0;
        let gyro_y = (i16::from(buffer[10]) << 8 | i16::from(buffer[11])) as f32 / 131.0;
        let gyro_z = (i16::from(buffer[12]) << 8 | i16::from(buffer[13])) as f32 / 131.0;

        Ok(AccelGyroData {
            accel_x,
            accel_y,
            accel_z,
            gyro_x,
            gyro_y,
            gyro_z,
        })
    }
}

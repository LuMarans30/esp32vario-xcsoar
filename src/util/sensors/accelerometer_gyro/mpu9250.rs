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

        Ok(())
    }

    fn get_measurement(&mut self) -> Result<Self::Data, EspError> {
        let mut buffer = [0u8; 14];
        let mut i2c = self.i2c.lock().unwrap();

        Ok(todo!())
    }
}

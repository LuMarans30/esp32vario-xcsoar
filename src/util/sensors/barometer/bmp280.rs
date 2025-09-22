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

        Ok(())
    }

    fn get_measurement(&mut self) -> Result<Self::Data, EspError> {
        let mut buffer = [0u8; 6];
        let mut i2c = self.i2c.lock().unwrap();

        Ok(todo!())
    }
}

use super::super::*;

#[derive(Debug, Clone)]
pub struct GpsData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub speed: f32,
}

pub struct Gps<'a> {
    address: u8,
    i2c: Arc<Mutex<I2cDriver<'a>>>,
}

impl<'a> Sensor<'a> for Gps<'a> {
    type Data = GpsData;

    fn new(i2c: Arc<Mutex<I2cDriver<'a>>>, address: u8) -> Self {
        Gps { address, i2c }
    }

    fn init(&mut self) -> Result<(), EspError> {
        let mut i2c = self.i2c.lock().unwrap();

        Ok(())
    }

    fn get_measurement(&mut self) -> Result<Self::Data, EspError> {
        let mut buffer = [0u8; 32];
        let mut i2c = self.i2c.lock().unwrap();

        Ok(todo!())
    }
}

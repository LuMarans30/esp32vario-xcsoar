pub struct NMEAData {
    latitude: f32,        // deg
    longitude: f32,       // deg
    altitude: f32,        // m
    speed: f32,           // m/s
    heading: f32,         // deg
    static_pressure: f32, // hPa
    temperature: f32,     // celsius degree
    average_cps: f32,     // m/s
    acceleration_x: f32,  // m/s^2
    acceleration_y: f32,  // m/s^2
    acceleration_z: f32,  // m/s^2
}

pub enum NMEASentenceType {
    POV,
    PEYA,
    PEYI,
}

impl NMEAData {
    pub fn new(
        latitude: f32,
        longitude: f32,
        altitude: f32,
        speed: f32,
        heading: f32,
        static_pressure: f32,
        temperature: f32,
        average_cps: f32,
        acceleration_x: f32,
        acceleration_y: f32,
        acceleration_z: f32,
    ) -> Self {
        NMEAData {
            latitude,
            longitude,
            altitude,
            speed,
            heading,
            static_pressure,
            temperature,
            average_cps,
            acceleration_x,
            acceleration_y,
            acceleration_z,
        }
    }

    pub fn get_data_string(&self, data_type: NMEASentenceType) -> String {
        let data = match data_type {
            NMEASentenceType::POV => self.get_pov(),
            NMEASentenceType::PEYA => self.get_peya(),
            NMEASentenceType::PEYI => self.get_peyi(),
        };

        let checksum = Self::nmea_checksum(&data);
        format!("{}{}\r\n", data, checksum)
    }

    /**
     * POV data:
     * - static pressure (hPa)
     * - temperature (°C)
     * - average CPS (m/s)
     */
    fn get_pov(&self) -> String {
        format!(
            "$POV,P,{:.2},T,{:.2},E,{:.2}*",
            self.static_pressure / 100.0,
            self.temperature,
            self.average_cps / 100.0
        )
    }

    /**
     * PEYA data:
     * - static pressure (hPa)
     * - altitude (m)
     * - average CPS (m/s)
     * - temperature (°C)
     */
    fn get_peya(&self) -> String {
        format!(
            "$PEYA,{:.2},,{:.2},,,,,{:.2},{:.2},*",
            self.static_pressure / 100.0,
            self.altitude / 100.0,
            self.average_cps / 100.0,
            self.temperature
        )
    }

    /**
     * PEYI data:
     * - bank (or roll) angle (°)
     * - pitch angle (°)
     * - acceleration (in the X, Y, and Z directions) (m/s^2)
     * - heading (°)
     */
    fn get_peyi(&self) -> String {
        format!(
            "$PEYI,{:.2},{:.2},,,,{:.2},{:.2},{:.2},,{:.2},*",
            self.heading,
            self.heading,
            self.acceleration_x / 100.0,
            self.acceleration_y / 100.0,
            self.acceleration_z / 100.0,
            self.heading
        )
    }

    fn nmea_checksum(msg: &str) -> u8 {
        msg.chars()
            .skip(1)
            .take_while(|c| *c != '*')
            .fold(0, |sum, c| sum ^ c as u8)
    }
}

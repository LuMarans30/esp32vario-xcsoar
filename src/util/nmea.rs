pub struct NMEAData {
    pub latitude: f32,        // deg
    pub longitude: f32,       // deg
    pub altitude: f32,        // m
    pub speed: f32,           // m/s
    pub heading: f32,         // deg
    pub static_pressure: f32, // hPa
    pub temperature: f32,     // celsius degree
    pub average_cps: f32,     // m/s
    pub acceleration_x: f32,  // m/s^2
    pub acceleration_y: f32,  // m/s^2
    pub acceleration_z: f32,  // m/s^2
}

impl NMEAData {
    pub fn get_data_string(&self, data_type: &str) -> String {
        let data = match data_type {
            "pov" => self.get_pov(),
            "peya" => self.get_peya(),
            "peyi" => self.get_peyi(),
            _ => String::from("Invalid data type"),
        };

        let checksum = self.nmea_checksum(&data);
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

    fn nmea_checksum(&self, msg: &str) -> u8 {
        msg.chars()
            .skip(1)
            .take_while(|c| *c != '*')
            .fold(0, |sum, c| sum ^ c as u8)
    }
}

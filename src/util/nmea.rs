pub struct Position {
    latitude: f32,  // deg
    longitude: f32, // deg
    altitude: f32,  // m
}

pub struct Motion {
    speed: f32,          // m/s
    heading: f32,        // deg
    acceleration_x: f32, // m/s^2
    acceleration_y: f32, // m/s^2
    acceleration_z: f32, // m/s^2
}

pub struct Environment {
    static_pressure: f32, // hPa
    temperature: f32,     // celsius degree
    average_cps: f32,     // m/s
}

pub struct NMEAData {
    position: Position,
    motion: Motion,
    environment: Environment,
}

pub enum NMEASentenceType {
    Pov,
    Peya,
    Peyi,
}

impl NMEAData {
    pub fn build_position(latitude: f32, longitude: f32, altitude: f32) -> Position {
        Position {
            latitude,
            longitude,
            altitude,
        }
    }

    pub fn build_motion(
        speed: f32,
        heading: f32,
        acceleration_x: f32,
        acceleration_y: f32,
        acceleration_z: f32,
    ) -> Motion {
        Motion {
            speed,
            heading,
            acceleration_x,
            acceleration_y,
            acceleration_z,
        }
    }

    pub fn build_environment(
        static_pressure: f32,
        temperature: f32,
        average_cps: f32,
    ) -> Environment {
        Environment {
            static_pressure,
            temperature,
            average_cps,
        }
    }

    pub fn new(position: Position, motion: Motion, environment: Environment) -> Self {
        Self {
            position,
            motion,
            environment,
        }
    }

    pub fn get_data_string(&self, data_type: NMEASentenceType) -> String {
        let data = match data_type {
            NMEASentenceType::Pov => self.get_pov(),
            NMEASentenceType::Peya => self.get_peya(),
            NMEASentenceType::Peyi => self.get_peyi(),
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
            self.environment.static_pressure / 100.0,
            self.environment.temperature,
            self.environment.average_cps / 100.0
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
            self.environment.static_pressure / 100.0,
            self.position.altitude / 100.0,
            self.environment.average_cps / 100.0,
            self.environment.temperature
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
            self.motion.heading,
            self.motion.heading,
            self.motion.acceleration_x / 100.0,
            self.motion.acceleration_y / 100.0,
            self.motion.acceleration_z / 100.0,
            self.motion.heading
        )
    }

    fn nmea_checksum(msg: &str) -> u8 {
        msg.chars()
            .skip(1)
            .take_while(|c| *c != '*')
            .fold(0, |sum, c| sum ^ c as u8)
    }
}

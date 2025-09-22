use esp_idf_hal::{
    delay::FreeRtos,
    io::{EspIOError, Write},
    prelude::*,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::{
        server::{Configuration, EspHttpServer},
        Method,
    },
    log::*,
    sys::*,
};

use crate::util::wifi;

use anyhow::{bail, Result};

mod util {
    pub mod nmea;
    pub mod sensors;
    pub mod wifi;
}

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

/**
 * - Initialize the sensor manager and the TCP client
 * - Send sensor data in NMEA format (via Wi-Fi) to the TCP server (XCSoar)
 */
fn main() -> Result<()> {
    link_patches();
    EspLogger::initialize_default();
    esp_idf_svc::nvs::EspDefaultNvsPartition::take()?;

    let peripherals = Peripherals::take().unwrap();
    let app_config = CONFIG;

    let sysloop = EspSystemEventLoop::take()?;

    let _wifi = match wifi::wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => {
            println!("Connected to Wi-Fi network!");
            inner
        }
        Err(err) => {
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    let mut server = EspHttpServer::new(&Configuration::default())?;

    server.fn_handler(
        "/",
        Method::Get,
        |request| -> core::result::Result<(), EspIOError> {
            let html = index_html();
            let mut response = request.into_ok_response()?;
            response.write_all(html.as_bytes())?;
            Ok(())
        },
    )?;

    loop {
        FreeRtos::delay_ms(1000);
    }
}

fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}

fn index_html() -> String {
    templated("Hello from ESP32 Open Variometer!")
}

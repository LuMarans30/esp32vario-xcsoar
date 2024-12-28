## Open-Source Variometer using ESP32S3 for XCSoar (WIP)

### Introduction
This project is an open-source variometer designed to work seamlessly with XCSoar, a popular flight planning and navigation software. 
Built around the ESP32S3 microcontroller, it leverages the power of Rust programming language and Espressif's standard libraries to ensure memory safety and high performance.

### Key Features

* Collects sensor data and converts it to NMEA format for compatibility with XCSoar
* Transmits NMEA data over TCP (Wi-Fi) to XCSoar-compatible devices, such as Android smartphones or Kobe eReader

### Getting Started
To learn more about this project and how to get started, please explore the following sections:

* [Installation](#installation)
* [Usage](#usage)
* [License](#license)

#### Installation

Change the SSID and Password in `src/util/tcp_client.rs` to your device hotspot Wi-Fi details.

You can flash this project on your ESP32S3 easily.
- `git clone https://github.com/LuMarans30/esp32vario-xcsoar.git`
- `cd esp32vario-xcsoar`
- `cargo build --release`
- `espflash flash target/xtensa-esp32s3-espidf/release/esp32vario-xcsoar`

#### Usage

The ESP32S3 will automatically connect to your phone via Wi-Fi.
To use the variometer as a device:
- open XCSoar
- Add a new device in the config page: `OpenVario on port 8880`

The variometer will start sending NMEA data to XCSoar over 8880.

#### License

This project is released under the [MIT License](https://opensource.org/licenses/MIT).

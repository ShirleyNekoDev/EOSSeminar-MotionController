# Requirements

- Arduino toolchain (or arduino-cli)
- Esp32 dev board definitions
  1. `arduino-cli config init`
  2. in `arduino-cli.yaml` add `https://raw.githubusercontent.com/espressif/arduino-esp32/gh-pages/package_esp32_dev_index.json` to `additional_urls`
  3. `arduino-cli core install esp32:esp32`

# Build

1. Init git submodule `git submodule update --init --recursive`
2. Go to project directory and create a `build` directory, go into it
3. Run `cmake -DCMAKE_TOOLCHAIN_FILE=../../arduino-toolchain/Arduino-toolchain.cmake ..`
4. In `BoardOptions.cmake`
  - Select board Esp32C3 `set(ARDUINO_BOARD "esp32.esp32c3")`
  - Set upload speed lower than default e.g. `set(ARDUINO_ESP32_ESP32C3_MENU_UPLOADSPEED_115200 TRUE)`
  - For development purposes, change the logging level e.g. (`set(ARDUINO_ESP32_ESP32C3_MENU_DEBUGLEVEL_VERBOSE TRUE)`)
5. Run `cmake ..`
6. Run `make <TARGET>`
7. Upload via `make upload TARGET=<TARGET> SERIAL_PORT=<PORT>` (find via `arduino-cli board list`)

# Caveats

## I2C

Seems not to be working with all GPIO pins. A working pair (SDA/SCL) of pins for the ESP32-C3 Dev Module we use were pins 9 and 10.

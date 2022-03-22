# Project-Specific BLE Services & Characteristics

- ~~BatteryService~~
- DIYMotionControllerService (`21aa4893-f368-46b5-977e-cdfc93555aaf`)
    - ClassicControlsCharacteristic (Button and Joystick) (`0385fe9d-56a6-40a4-b055-9b610cfcfe0c`)
    - FeedbackCharacteristic (LED & Rumble) (`2f99ce60-cafe-4e44-9633-5bb3aea5d6c1`)
    - MotionControlsCharacteristic (Orientation and Acceleration) (`e4b0e66b-f9ac-424b-ae72-59f7fbadf871`)

# Pin Mapping for ESP32-C3-32S (Devkit)

- I2C SDA: IO9 (tested)
- I2C SCL: I010 (tested)
- JOYSTICK_X: IO2/ADC1_CH2
- JOYSTICK_Y: IO1/ADC1_CH1
- LED_R: IO3 (fixed)
- LED_G: IO4 (fixed)
- LED_B: IO5 (fixed)
- BUTTON_MENU: IO18
- BUTTON_A: IO7
- BUTTON_B: IO8
- RUMBLE_CONTROL: IO6

# Known issues
- Once disconnected from the controller, you can neither discover it during a
  scan, nor connect to it again. This is a [known bug of the ESP32 Arduino
  Core](https://github.com/espressif/arduino-esp32/issues/6016).

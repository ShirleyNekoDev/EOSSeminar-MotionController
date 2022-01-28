#define BLE_ENABLED
#define LOG_LOCAL_LEVEL ESP_LOG_DEBUG
#include "esp_log.h"

#include "ble/classic_characteristic.h"
#include "ble_manager.h"
#include "board_definitions.h"
#include "services/button_service.h"
#include "services/joystick_service.h"
#include "version.h"
#include <Arduino.h>
#include <Wire.h>

static const char *TAG = "MAIN";

// TODO put this into own BMI160 "module"
void checkIMUConnection() {
  Wire.beginTransmission(0x68);
  Wire.write(0x00);
  Wire.endTransmission(false);
  Wire.requestFrom(static_cast<uint8_t>(0x68), static_cast<size_t>(1), true);
  if (Wire.read() != 0xD1) {
    ESP_LOGE(TAG, "BMI160 connection not working.");
  }
  ESP_LOGD(TAG, "BMI160 connection successful.");
}

using namespace dmc;

namespace {
button::Status button_status;
joystick::Status joystick_status;
ble::ClassicControlsCharacteristic *cc_ch;
} // namespace

void setup() {
  // Initialize logging
  esp_log_level_set(TAG, LOG_LOCAL_LEVEL);

  ESP_LOGI(TAG, "Starting up firmware v%i.%i...", VERSION_MAJOR, VERSION_MINOR);

  // Initialize I2C Port
  pinMode(PIN_I2C_SDA, INPUT_PULLUP);
  pinMode(PIN_I2C_SCL, INPUT_PULLUP);
  if (!Wire.begin(static_cast<int>(PIN_I2C_SDA), static_cast<int>(PIN_I2C_SCL),
                  static_cast<uint32_t>(400000))) {
    ESP_LOGE(TAG, "Initialization of I2C Bus failed");
  }
  ESP_LOGD(TAG, "I2C has been initialized.");

  // Check connection to BMI160 IMU
  checkIMUConnection();

  // Setup buttons
  button::start();

  // Setup joystick
  joystick::start();

  // Initialize bluetooth
  ble::initialize();

  // Start the characteristics
  cc_ch = new ble::ClassicControlsCharacteristic();

  ble::start();

  ESP_LOGI(TAG, "Setup complete. Now entering Arduino Core loop.");
}


void loop() {
  { // Classic controls
    ESP_LOGV(TAG, "Refreshing classic controls...");
    button::refresh();
    joystick::refresh();
    bool update_required = false;
    if (button::read_status(button_status)) {
      update_required = true;
      ESP_LOGD(TAG, "Button status was updated");
    }
    if (joystick::read_status(joystick_status)) {
      update_required = true;
      ESP_LOGD(TAG, "Joystick status was updated");
    }

    if (update_required) {
      ESP_LOGD(TAG, "Issuing an update to the CC Characteristic.");
      cc_ch->update(button_status, joystick_status);
    }
  }

  delay(100);

  // TODO build an abstraction for state management
  // TODO switch between device states CONNECTED and PAIRING in reaction to
  // events from the BLEServer Callback handlers
}

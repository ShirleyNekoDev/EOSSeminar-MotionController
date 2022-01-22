#define LOG_LOCAL_LEVEL ESP_LOG_VERBOSE
#include "esp_log.h"

#include "ble_manager.h"
#include "pin_definitions.h"
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

void setup() {
  // Initialize logging
  esp_log_level_set(TAG, LOG_LOCAL_LEVEL);

  ESP_LOGI(TAG, "Starting up firmware v%i.%i...", VERSION_MAJOR, VERSION_MINOR);

  // Initialize I2C Port
  if (!Wire.begin(static_cast<int>(PIN_I2C_SDA), static_cast<int>(PIN_I2C_SCL),
                  static_cast<uint32_t>(400000))) {
    ESP_LOGE(TAG, "Initialization of I2C Bus failed");
  }
  ESP_LOGD(TAG, "I2C has been initialized.");

  // Check connection to BMI160 IMU
  checkIMUConnection();

  // Setup bluetooth
  dcm::ble::start();

  ESP_LOGI(TAG, "Setup complete. Now entering Arduino Core loop.");
}

void loop() {

}

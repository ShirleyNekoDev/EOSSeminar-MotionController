#define LOG_LOCAL_LEVEL ESP_LOG_VERBOSE
#include "esp_log.h"

#include <Arduino.h>
#include <Wire.h>

static const char *TAG = "MainModule";

#define BMI160_I2C 0x68
#define BMI160_CHIP_ID 0xd1
#define BMI160_REG_CHIP_ID 0x00
#define BMI160_REG_CMD 0x7e
#define BMI160_CMD_SOFTRESET 0xb6
#define BMI160_CMD_PMUSETACC 0x10
#define BMI160_PMUSTATE_ACC_NORMAL 0x01
void bmi160_write(uint8_t *buf, uint8_t length) {
  Wire.beginTransmission(BMI160_I2C);
  for (uint8_t i = 0; i < length; i++) {
    Wire.write(buf[i]);
  }
  Wire.endTransmission();
}

void bmi160_send_command(uint8_t cmd) {
  uint8_t buf[2];
  buf[0] = BMI160_REG_CMD;
  buf[1] = cmd;
  bmi160_write(buf, 2);
}

void bmi160_read(uint8_t address, uint8_t *result, uint8_t nbytes) {
  Wire.beginTransmission(BMI160_I2C);
  Wire.write(address);
  Wire.endTransmission(false);
  Wire.requestFrom((uint8_t)BMI160_I2C, (uint8_t)nbytes, (uint8_t) true);
  *result = Wire.read();
}

bool bmi160_test_connection() {
  uint8_t result = 0;
  bmi160_read(BMI160_REG_CHIP_ID, &result, 1);
  if (result != BMI160_CHIP_ID) {
    ESP_LOGE(TAG, "Connection is not ok, returned chipid=%X", result);
    return false;
  }
  ESP_LOGI(TAG, "Connection to BMI160 successful");
  return true;
}

void bmi160_read_acc(uint16_t *result) {
  uint8_t buf[6];
  bmi160_read(0x12, buf, 6);
  for (uint8_t i = 0; i < 3; i++) {
    result[i] = 0;
    for (uint8_t j = 0; j < 2; j++) {
      result[i] |= (buf[2 * i + j] << 8 * j);
    }
  }
}

void bmi160_init() {
  // wait for a working connection to BMI160
  while (!bmi160_test_connection()) {
    delay(500);
  }

  // softreset
  bmi160_send_command(BMI160_CMD_SOFTRESET);
  delay(100);

  // enable accelerometer
  bmi160_send_command(BMI160_CMD_PMUSETACC | BMI160_PMUSTATE_ACC_NORMAL);
  delay(5);
}

void setup() {
  // initialize I2C bus
  Wire.begin((int)1, (int)19, (uint32_t)100000);

  // initialize BMI160 chip
  bmi160_init();

  // Set log level
  esp_log_level_set(TAG, ESP_LOG_VERBOSE);
}

void readLoop() {
  ESP_LOGD(TAG, "Reading BMI160 now...");
  uint16_t acc[3];
  bmi160_read_acc(acc);
  ESP_LOGD(TAG, "Read accelerometer values X: %d, Y: %d, Z: %d\n", acc[0],
           acc[1], acc[2]);
}

void loop() {
  delay(1000);
  readLoop();
}

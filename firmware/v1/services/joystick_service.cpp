#include "services/joystick_service.h"
#include "board_definitions.h"
#include "constants.h"
#include "math_utils.h"

#define LOG_LOCAL_LEVEL ESP_LOG_DEBUG
#include "esp_log.h"
#include <Arduino.h>
#include <math.h>

static const char *TAG = "JOYSTICK_SERVICE";

#define ADC_MIN_VALUE 0
#define ADC_MAX_VALUE ((1 << ADC_BIT_RESOLUTION) - 1)

#define MIN_DIFF_SIGNIFICANT 0.1f

namespace dmc {

namespace joystick {

namespace {

int16_t x_raw;
int16_t y_raw;

int16_t x_offset = 0.0f;
int16_t y_offset = 0.0f;

float x_norm_factor[2];
float y_norm_factor[2];

void calibrate() {
  ESP_LOGD(TAG, "Calibrating joystick...");
  refresh();

  // X
  x_offset = -x_raw;
  x_norm_factor[0] = 1.0f / (ADC_MIN_VALUE - x_offset); // negative
  x_norm_factor[1] = 1.0f / (ADC_MAX_VALUE + x_offset); // positive

  // Y
  y_offset = -y_raw;
  y_norm_factor[0] = 1.0f / (ADC_MIN_VALUE - y_offset); // negative
  y_norm_factor[1] = 1.0f / (ADC_MAX_VALUE + y_offset); // positive
  ESP_LOGD(TAG, "Joystick calibration done.");
}

} // namespace

void start() {
  esp_log_level_set(TAG, LOG_LOCAL_LEVEL);
  calibrate();
}

void refresh() {
  ESP_LOGV(TAG, "Refreshing joystick data.");
  x_raw = analogRead(PIN_JOYSTICK_X);
  y_raw = analogRead(PIN_JOYSTICK_Y);
}

float get_x() {
  int16_t x = x_raw + x_offset;
  return x * x_norm_factor[x >= 0];
}

float get_y() {
  int16_t y = y_raw + y_offset;
  return y * y_norm_factor[y >= 0];
}

bool read_status(Status &joystick_status) {
  ESP_LOGV(TAG, "Reading joystick data to status.");

  static float last_x = 0;
  static float last_y = 0;

  float new_x = get_x();
  float new_y = get_y();

  ESP_LOGV(TAG, "Read data:\t%f\t%f", new_x, new_y);

  uint16_t new_x_packed = pack_float(new_x);
  uint16_t new_y_packed = pack_float(new_y);

  float max_diff = fabs(new_x - last_x);
  if (max_diff < fabs(new_y - last_y))
    max_diff = fabs(new_y - last_y);

  bool update_occurred = false;
  if (joystick_status.x != new_x) {
    ESP_LOGV(TAG, "X updated: old: %i new: %i", joystick_status.x, new_x);
    joystick_status.x = new_x_packed;
    update_occurred = true;
  }

  if (joystick_status.y != new_y) {
    ESP_LOGV(TAG, "Y updated: old: %i new: %i", joystick_status.y, new_y);
    joystick_status.y = new_y_packed;
    update_occurred = true;
  }

  if (update_occurred && max_diff >= MIN_DIFF_SIGNIFICANT) {
    // If the update was significant
    last_x = new_x;
    last_y = new_y;
    ESP_LOGD(TAG, "A significant update of the joystick data occurred.");
    return true;
  }

  return false;
}

} // namespace joystick

} // namespace dmc

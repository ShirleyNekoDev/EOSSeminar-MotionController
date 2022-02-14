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

  ESP_LOGD(TAG, "OFFSETS: %i %i (max = %i)", x_raw, y_raw, ADC_MAX_VALUE);
}

} // namespace

void start() {
  esp_log_level_set(TAG, LOG_LOCAL_LEVEL);
  calibrate();
}

void refresh() {
  ESP_LOGV(TAG, "Refreshing joystick data.");
  // Multisampling to reduce noise
  x_raw = 0;
  x_raw += analogRead(PIN_JOYSTICK_X);
  x_raw += analogRead(PIN_JOYSTICK_X);
  x_raw += analogRead(PIN_JOYSTICK_X);
  x_raw += analogRead(PIN_JOYSTICK_X);
  x_raw /= 4;
  y_raw = 0;
  y_raw += analogRead(PIN_JOYSTICK_Y);
  y_raw += analogRead(PIN_JOYSTICK_Y);
  y_raw += analogRead(PIN_JOYSTICK_Y);
  y_raw += analogRead(PIN_JOYSTICK_Y);
  y_raw /= 4;
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
  static int _calls = 0;
  ESP_LOGV(TAG, "Reading joystick data to status.");

  static float last_x = 0;
  static float last_y = 0;

  float new_x = get_x();
  float new_y = get_y();

  ESP_LOGV(TAG, "Read data:\t%f\t%f", new_x, new_y);

  float max_diff = fabs(new_x - last_x);
  if (max_diff < fabs(new_y - last_y))
    max_diff = fabs(new_y - last_y);

  // Aggressive filtering of 1.0 readings
  if (new_x > 1.0 || new_y > 1.0)
    return false;

  static uint8_t maxed_out_samples[2] = {0, 0};
  if (new_x >= 1.0) {
    if (++maxed_out_samples[0] < 5) {
      new_x = last_x;
    }
  } else {
    maxed_out_samples[0] = 0;
  }
  if (new_y >= 1.0) {
    if (++maxed_out_samples[1] < 5) {
      new_y = last_y;
    }
  } else {
    maxed_out_samples[1] = 0;
  }

  if (last_x >= -0.1 && last_x <= 0.1 && new_x >= 1.0)
    return false;
  if (last_y >= -0.1 && last_y <= 0.1 && new_y >= 1.0)
    return false;

  // Apparently the ADC sometimes reads garbage values or BLE notifications
  // are dropped, so we have to occasionally have to send the valid
  // data, even if in theory they have not updated.
  if ((max_diff >= MIN_DIFF_SIGNIFICANT) || (++_calls % 128 == 0)) {
    // If the update was significant
    if (new_x >= 1.0 || new_y >= 1.0) {
      ESP_LOGW(TAG, "Suspicious values of joystick: %f %f", new_x, new_y);
    }
    last_x = new_x;
    last_y = new_y;
    joystick_status.x = pack_float(new_x);
    joystick_status.y = pack_float(new_y);
    if (_calls % 128 != 0)
      ESP_LOGD(TAG, "A significant update of the joystick data occurred.");
    return true;
  }

  return false;
}

} // namespace joystick

} // namespace dmc

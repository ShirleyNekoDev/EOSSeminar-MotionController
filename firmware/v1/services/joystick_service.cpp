#include "services/joystick_service.h"
#include "math_utils.h"
#include "pin_definitions.h"

#include <Arduino.h>

#define ADC_BIT_RESOLUTION 10
#define ADC_MIN_VALUE 0
#define ADC_MAX_VALUE ((1 << ADC_BIT_RESOLUTION) - 1)

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
  refresh();

  // X
  x_offset = -x_raw;
  x_norm_factor[0] = 1.0f / (ADC_MIN_VALUE - x_offset); // negative
  x_norm_factor[1] = 1.0f / (ADC_MAX_VALUE + x_offset); // positive

  // Y
  y_offset = -y_raw;
  y_norm_factor[0] = 1.0f / (ADC_MIN_VALUE - y_offset); // negative
  y_norm_factor[1] = 1.0f / (ADC_MAX_VALUE + y_offset); // positive
}

} // namespace

void start() { calibrate(); }

void refresh() {
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

void read_status(Vec2 &joystick_status) {
  joystick_status.x = pack_float(get_x());
  joystick_status.y = pack_float(get_y());
}

} // namespace joystick

} // namespace dmc

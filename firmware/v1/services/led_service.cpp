#include "services/led_service.h"
#include "board_definitions.h"

#include <Arduino.h>

namespace dmc {

namespace led {

void start() {
  // Attach led pins to pwm channels
  ledcAttachPin(PIN_LED_R, 1);
  ledcAttachPin(PIN_LED_G, 2);
  ledcAttachPin(PIN_LED_B, 3);

  // initialize channels
  ledcSetup(1, 12000, 8); // 12 kHz, 8bit resolution
  ledcSetup(2, 12000, 8); // 12 kHz, 8bit resolution
  ledcSetup(3, 12000, 8); // 12 kHz, 8bit resolution
  // TODO put magic numbers somewhere else
}

void refresh() {
  // no operation for now
}

void write_status(Status &led_status) {
  ledcWrite(1, led_status.r);
  ledcWrite(2, led_status.g);
  ledcWrite(3, led_status.b);
}

} // namespace led

} // namespace dmc

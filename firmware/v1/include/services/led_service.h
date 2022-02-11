#ifndef LED_SERVICE_H__
#define LED_SERVICE_H__

#include <stdint.h>

namespace dmc {

namespace led {

struct Status {
  uint8_t r = 0;
  uint8_t g = 0;
  uint8_t b = 0;
};

void start();
void refresh();

void write_status(Status &led_status);

} // namespace led

} // namespace dmc

#endif

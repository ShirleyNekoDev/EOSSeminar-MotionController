#include <cstdint>

enum LedBlinkMode {
  ONCE = 1,
  THRICE = 2,
  CONTINUOUS = 3
};
enum LedBlinkSpeed {
  FAST = true,
  SLOW = false,
};

void service_led_set_color(uint8_t red, uint8_t green, uint8_t blue) {
  // TODO
}

void service_led_off() {
  // TODO
}

void service_led_blink(LedBlinkMode mode, LedBlinkSpeed speed) {
  // TODO
}

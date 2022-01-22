#include <stdint.h>

const uint16_t RUMBLE_BURST_LENGTH_SHORT = 250; // in ms
const uint16_t RUMBLE_BURST_LENGTH_LONG = 750; // in ms

enum RumbleBurstMode {
  OFF = 0,
  SHORT = 1,
  LONG = 2
};

void service_rumble_start(uint8_t intensity) {
  // TODO
}

void service_rumble_stop() {
  // TODO
}

void service_rumble_burst(RumbleBurstMode mode, uint8_t intensity = 0xFF) {
  // TODO
}

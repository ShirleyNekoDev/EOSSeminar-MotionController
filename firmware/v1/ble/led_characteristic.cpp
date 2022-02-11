
#include "ble/led_characteristic.h"
#include "ble_abstraction.h"
#include "constants.h"
#include "services/led_service.h"
#include <assert.h>

#define LOG_LOCAL_LEVEL ESP_LOG_DEBUG
static const char *TAG = "LED_CH";
#include "esp_log.h"

namespace dmc {

namespace ble {

LedCharacteristic::LedCharacteristic()
    : dmc::ble::DMCCharacteristic("Led", FEEDBACK_CH_UUID, false, true, true,
                                  sizeof(buffer_)) {
  static_assert(sizeof(buffer_) < 20,
                "Bluetooth notifications cannot exceed 20 bytes");
  // TODO get this from somewhere else
  write(reinterpret_cast<uint8_t *>(&buffer_), sizeof(buffer_));
}

bool LedCharacteristic::read_update(led::Status &led_status) {
  read(reinterpret_cast<uint8_t *>(&buffer_), sizeof(buffer_));
  bool was_updated = false;
  if (led_status.r != buffer_.led_status.r ||
      led_status.g != buffer_.led_status.g ||
      led_status.b != buffer_.led_status.b) {
    ESP_LOGD(TAG, "The Led Characteristic was updated.");
    was_updated = true;
  }
  led_status = buffer_.led_status;
  return was_updated;
}
} // namespace ble

} // namespace dmc

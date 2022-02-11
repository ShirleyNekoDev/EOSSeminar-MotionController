#include "ble/classic_characteristic.h"

#include "ble/classic_characteristic.h"
#include "ble_abstraction.h"
#include "constants.h"
#include "services/button_service.h"
#include "services/joystick_service.h"
#include <assert.h>

#define LOG_LOCAL_LEVEL ESP_LOG_DEBUG
static const char *TAG = "CLASSIC_CH";
#include "esp_log.h"

namespace dmc {

namespace ble {

ClassicControlsCharacteristic::ClassicControlsCharacteristic()
    : dmc::ble::DMCCharacteristic("ClassicControls", CLASSIC_CH_UUID, true,
                                  false, true, sizeof(buffer_)) {
  static_assert(sizeof(buffer_) < 20,
                "Bluetooth notifications cannot exceed 20 bytes");
  write(reinterpret_cast<uint8_t *>(&buffer_), sizeof(buffer_));
}
void ClassicControlsCharacteristic::update(button::Status &button_status,
                                           joystick::Status &joystick_status) {
  buffer_.button_status = button_status;
  buffer_.joystick_status = joystick_status;
  ESP_LOGD(TAG, "Writing update to BLE characteristic.");
  write(reinterpret_cast<uint8_t *>(&buffer_), sizeof(buffer_));
}
} // namespace ble

} // namespace dmc

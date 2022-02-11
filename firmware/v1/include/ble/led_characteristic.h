#ifndef LED_CHARACTERISTIC_H__
#define LED_CHARACTERISTIC_H__

#include "ble_abstraction.h"
#include "services/button_service.h"
#include "services/joystick_service.h"
#include "services/led_service.h"

namespace dmc {

namespace ble {

class LedCharacteristic : public DMCCharacteristic {
public:
  explicit LedCharacteristic();
  bool read_update(led::Status &led_status);

private:
  struct {
    led::Status led_status;
  } buffer_;
};
}
}

#endif

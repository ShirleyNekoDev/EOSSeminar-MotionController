#ifndef CLASSIC_CHARACTERISTIC_H__
#define CLASSIC_CHARACTERISTIC_H__

#include "ble_abstraction.h"
#include "services/button_service.h"
#include "services/joystick_service.h"

namespace dmc {

namespace ble {

class ClassicControlsCharacteristic : public DMCCharacteristic {
public:
  explicit ClassicControlsCharacteristic();
  void update(button::Status &button_status, joystick::Status &joystick_status);

private:
  struct {
    joystick::Status joystick_status;
    button::Status button_status;
  } buffer_;
};
}
}

#endif

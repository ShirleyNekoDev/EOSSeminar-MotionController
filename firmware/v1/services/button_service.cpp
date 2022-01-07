#include <cstdint>
#include "math_utils.cpp"

// struct ControllerButtonStatus { // 1 Byte
//   bool x: 1;
//   bool y: 1;
//   bool a: 1;
//   bool b: 1;
//   bool start: 1;
//   bool menu: 1;
// };
struct ButtonStatus {
  uint16_t trigger;
  bool a: 1;
  bool menu: 1;
};

void service_button_read_status(ButtonStatus& button_status) {
  // TODO
  button_status.trigger = pack_float(0.0f);
  button_status.a = false;
  button_status.menu = false;
}

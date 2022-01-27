#ifndef BUTTON_SERVICE_H__
#define BUTTON_SERVICE_H__

#include <stdint.h>

namespace dmc {

namespace button {

// struct ControllerButtonStatus { // 1 Byte
//   bool x: 1;
//   bool y: 1;
//   bool a: 1;
//   bool b: 1;
//   bool start: 1;
//   bool menu: 1;
// };
struct Status {
  bool a : 1;
  bool b : 1;
  bool menu : 1;
};

void start();

void refresh();

bool read_status(Status &button_status);
}

} // namespace dmc

#endif

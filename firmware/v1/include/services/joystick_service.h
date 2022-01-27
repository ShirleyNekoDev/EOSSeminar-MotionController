#ifndef JOYSTICK_SERVICE_H__
#define JOYSTICK_SERVICE_H__

#include "math_utils.h"
#include <stdint.h>

namespace dmc {

namespace joystick {

using Status = Vec2;

void start();
void refresh();

float get_x();
float get_y();

bool read_status(Vec2 &joystick_status);

} // namespace joystick

} // namespace dmc

#endif

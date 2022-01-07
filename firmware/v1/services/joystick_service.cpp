#include "math_utils.cpp"

void service_joystick_read_status(Vec2& joystick_status) {
  // TODO
  float x = 0.0f;
  float y = 0.0f;
  joystick_status.x = pack_float(x);
  joystick_status.y = pack_float(y);
}

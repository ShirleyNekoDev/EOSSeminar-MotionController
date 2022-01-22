#ifndef PIN_DEFINITIONS_H__
#define PIN_DEFINITIONS_H__

#include <Arduino.h>

// TODO as these are a bit mixed right now, we should have some kind of #if
// #else things here, depending on the controller we want to compile against.
// maybe we can use some varible exposed by the arduino toolchain, like
// ARDUINO_BOARD

#define PIN_I2C_SDA 9
#define PIN_I2C_SCL 10

#define PIN_JOYSTICK_X A0
#define PIN_JOYSTICK_Y A1

#endif

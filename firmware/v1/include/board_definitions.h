#ifndef BOARD_DEFINITIONS_H__
#define BOARD_DEFINITIONS_H__

#if defined(ARDUINO_AVR_NANO)
#include "boards/avr_nano.h"
#elif defined(ARDUINO_ESP32C3_DEV) && defined(BOARD_PROTOTYPE_2)
#include "boards/esp32_esp32c3_prototype2.h"
#elif defined(ARDUINO_ESP32C3_DEV)
#include "boards/esp32_esp32c3.h"
#endif

#endif

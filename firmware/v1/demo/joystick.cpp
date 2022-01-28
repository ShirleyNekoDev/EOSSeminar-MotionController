#include <Arduino.h>

#include "board_definitions.h"
#include "services/joystick_service.h"

void setup() {
  Serial.begin(BOARD_BAUDRATE);
  dmc::joystick::start();
}

void loop() {
  dmc::joystick::refresh();
  Serial.print(dmc::joystick::get_x());
  Serial.print("\t");
  Serial.println(dmc::joystick::get_y());
  delay(1000);
}

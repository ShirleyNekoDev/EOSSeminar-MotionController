#include <Arduino.h>

#include "services/joystick_service.h"

void setup() {
  Serial.begin(9600);
  dmc::joystick::start();
}

void loop() {
  dmc::joystick::refresh();
  Serial.print(dmc::joystick::get_x());
  Serial.print("\t");
  Serial.println(dmc::joystick::get_y());
  delay(1000);
}

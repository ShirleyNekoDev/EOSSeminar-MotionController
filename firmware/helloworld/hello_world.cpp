//
// Arduino's hello world: Blink!
//

#include "Arduino.h"

void setup() {
	// initialize serial communication at 9600 bits per second:
	Serial.begin(9600);

	// print out hello world
	Serial.println("Hello World");

	// Setup to blink the inbuilt LED
#ifdef LED_BUILTIN
	pinMode(LED_BUILTIN, OUTPUT);
#endif
}

void loop() {
	// Blink the inbuilt LED
#ifdef LED_BUILTIN
  digitalWrite(LED_BUILTIN, HIGH);   // turn the LED on (HIGH is the voltage level)
  delay(1000);                       // wait for a second
  digitalWrite(LED_BUILTIN, LOW);    // turn the LED off by making the voltage LOW
  delay(1000);                       // wait for a second
#endif
}

## Requirements

- `cmake`
- Arduino IDE with all Board Definitions added that you might need.

To compile (and flash) this, create a `build` folder and inside execute
`cmake -DCMAKE_TOOLCHAIN_FILE=../../arduino-toolchain/Arduino-toolchain.cmake
..`.

The first time executing this will fail, because you have not yet selected a
board. Do so by editing the freshly created `BoardOptions.cmake` file or using
one of the other methods given in the error message.

When you are done, execute `cmake` again like above, which should succeed this time.

You now have the generated Makefile with the targets `hello_world` and `upload-hello_world` (and more). Different platforms / boards can require manual configuration at this point (e.g. adding a custom partition table for the ESP32).

The latter can be used by specifying a serial port like this: `make upload-hello_world SERIAL_PORT=/dev/<serial_port>` and should compile and flash the project onto the controller.

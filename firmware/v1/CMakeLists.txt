cmake_minimum_required(VERSION 3.0.0)

project(firmware_v1 CXX)

add_executable(firmware_v1 main.cpp)

# Arduino.h is included in hello_world.cpp, so link with Arduino core
# if, say, we included Wire.h, we would add "Wire" here as library to link
target_link_arduino_libraries(firmware_v1 PRIVATE core)

# This is needed for the generation of HEX binary and uploading it
target_enable_arduino_upload(firmware_v1)

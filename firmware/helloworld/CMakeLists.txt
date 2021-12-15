cmake_minimum_required(VERSION 3.0.0)

project(hello_world CXX)

add_executable(hello_world hello_world.cpp)

# Arduino.h is included in hello_world.cpp, so link with Arduino core
# if, say, we included Wire.h, we would add "Wire" here as library to link
target_link_arduino_libraries(hello_world PRIVATE core)

# This is needed for the generation of HEX binary and uploading it
target_enable_arduino_upload(hello_world)


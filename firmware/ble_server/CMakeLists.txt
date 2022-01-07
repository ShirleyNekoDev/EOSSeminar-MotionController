cmake_minimum_required(VERSION 3.0.0)

project(ble_server CXX)

add_executable(ble_server main.cpp)

# Arduino.h is included in ble_server.cpp, so link with Arduino core
# if, say, we included Wire.h, we would add "Wire" here as library to link
target_link_arduino_libraries(ble_server PRIVATE core)

# This is needed for the generation of HEX binary and uploading it
target_enable_arduino_upload(ble_server)


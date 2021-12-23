cmake_minimum_required(VERSION 3.0.0)

project(imu_test CXX)

add_executable(imu_test main.cpp)

# Arduino.h is included in main.cpp, so link with Arduino core
# if, say, we included Wire.h, we would add "Wire" here as library to link
target_link_arduino_libraries(imu_test PRIVATE core)

# This is needed for the generation of HEX binary and uploading it
target_enable_arduino_upload(imu_test)


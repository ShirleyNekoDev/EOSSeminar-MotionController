cmake_minimum_required(VERSION 3.0.0)

project(firmware_v1 CXX)

list(APPEND ARDUINO_LIBRARIES_SEARCH_PATHS_EXTRA
    "${CMAKE_CURRENT_SOURCE_DIR}/.."
)

add_subdirectory(ble)
add_subdirectory(services)

add_executable(firmware_v1
    main.cpp
    math_utils.cpp
    ble_manager.cpp
)
target_include_directories(firmware_v1 PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

target_link_libraries(firmware_v1 PRIVATE services_v1 ble_v1)

# Arduino.h is included in hello_world.cpp, so link with Arduino core
# if, say, we included Wire.h, we would add "Wire" here as library to link
target_link_arduino_libraries(firmware_v1 PRIVATE Wire core)

# This is needed for the generation of HEX binary and uploading it
target_enable_arduino_upload(firmware_v1)


###### Joystick Demo
add_subdirectory(demo)

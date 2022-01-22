add_executable(joystick_demo
    joystick.cpp
)
target_include_directories(joystick_demo PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)
target_link_libraries(joystick_demo PRIVATE joystick_service_v1)
target_link_arduino_libraries(joystick_demo PRIVATE core)
target_enable_arduino_upload(joystick_demo)

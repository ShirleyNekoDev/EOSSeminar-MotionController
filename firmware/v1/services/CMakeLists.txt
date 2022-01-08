add_library(v1_services
    battery_service.cpp
    button_service.cpp
    joystick_service.cpp
    led_service.cpp
    rumble_service.cpp
)
target_include_directories(v1_services PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)

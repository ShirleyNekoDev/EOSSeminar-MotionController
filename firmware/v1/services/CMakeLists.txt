add_library(services_v1
    battery_service.cpp
    button_service.cpp
    led_service.cpp
    rumble_service.cpp
)
target_include_directories(services_v1 PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)
target_link_libraries(services_v1 PUBLIC
    joystick_service_v1
)

# joystick service
add_library(joystick_service_v1
    joystick_service.cpp
)
target_include_directories(joystick_service_v1 PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)
target_link_arduino_libraries(joystick_service_v1 PUBLIC core)

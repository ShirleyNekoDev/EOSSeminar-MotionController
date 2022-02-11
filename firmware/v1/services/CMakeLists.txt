add_library(services_v1
    battery_service.cpp
    rumble_service.cpp
)
target_include_directories(services_v1 PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)
target_link_libraries(services_v1 PUBLIC
    button_service_v1
    joystick_service_v1
    led_service_v1
)

# button service
add_library(button_service_v1
    button_service.cpp
)
target_include_directories(button_service_v1 PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)
target_link_arduino_libraries(button_service_v1 PUBLIC Bounce2 core)

# joystick service
add_library(joystick_service_v1
    joystick_service.cpp
)
target_include_directories(joystick_service_v1 PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)
target_link_arduino_libraries(joystick_service_v1 PUBLIC core)

# led service
add_library(led_service_v1
    led_service.cpp
)
target_include_directories(led_service_v1 PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)
target_link_arduino_libraries(led_service_v1 PUBLIC core)

add_library(ble_v1
    classic_characteristic.cpp
)
target_include_directories(ble_v1 PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)
target_link_arduino_libraries(ble_v1 PUBLIC core)

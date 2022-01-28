#ifndef BLE_MANAGER_H__
#define BLE_MANAGER_H__

#include <BLE2902.h>
#include <BLECharacteristic.h>
#include <BLEDevice.h>
#include <BLEServer.h>
#include <BLEUtils.h>

namespace dmc {

namespace ble {

namespace {
extern bool clientConnected;
}

void initialize();
void start();

BLEService *get_dmc_service();
} // namespace ble

} // namespace dmc

#endif

#include "ble_manager.h"

#include "esp_log.h"
static const char *TAG = "BLE_MANAGER";

#include <BLE2902.h>
#include <BLEDevice.h>
#include <BLEServer.h>
#include <BLEUtils.h>

namespace {
constexpr const char *GAP_SERVICE_UUID = "00001800-0000-1000-8000-00805f9b34fb";
}

namespace dcm {

namespace ble {

BLEServer *pServer = NULL;
BLEService *genericAccessProfileService = NULL;

void start() {
  ESP_LOGD(TAG, "Initializing BLE.");

  BLEDevice::init("DIYMotionController");
  pServer = BLEDevice::createServer();

  // TODO set callbacks
  // TODO create services
  // TODO start advertising
}

} // namespace ble

} // namespace dcm

#include "ble_manager.h"
#include "ble_abstraction.h"
#include "constants.h"

#include "esp_log.h"
static const char *TAG = "BLE_MANAGER";

#include <BLE2902.h>
#include <BLEDevice.h>
#include <BLEServer.h>
#include <BLEUtils.h>

namespace dmc {

namespace ble {

namespace {
BLEServer *server;
BLEService *dmc_service;

bool clientConnected = false;

class MyServerCallbacks : public BLEServerCallbacks {
public:
  void onConnect(BLEServer *) override {
    clientConnected = true;
    // TODO initiate state change somehow
  }
  void onDisconnect(BLEServer *) override {
    clientConnected = false;
    // TODO initiate state change somehow
  }
};
}

BLEService *get_dmc_service() { return dmc_service; }

void initialize() {
  ESP_LOGD(TAG, "Initializing BLE.");

  BLEDevice::init(DEVICE_NAME);
  server = BLEDevice::createServer();

  server->setCallbacks(new BLEServerCallbacks());
  // TODO create services
  ESP_LOGD(TAG, "Creating DMC BLE Service with UUID %s", DMC_SERVICE_UUID);
  dmc_service = server->createService(DMC_SERVICE_UUID);
}

void start() {
  dmc_service->start();
  server->getAdvertising()->start();
}

DMCCharacteristic::DMCCharacteristic(const char *name, const char *uuid,
                                     bool read, bool write, bool notify)
    : name_{name} {
  properties_ = 0;
  if (read)
    properties_ |= BLECharacteristic::PROPERTY_READ;
  if (write)
    properties_ |= BLECharacteristic::PROPERTY_WRITE;
  if (notify)
    properties_ |= BLECharacteristic::PROPERTY_NOTIFY;

  characteristic_ = get_dmc_service()->createCharacteristic(uuid, properties_);
  characteristic_->addDescriptor(new BLE2902());
}

void DMCCharacteristic::write(uint8_t *data, size_t length) {
  if (clientConnected) {
    ESP_LOGD(TAG, "Writing %i bytes to characteristic \"%s\"", length, name_);
    characteristic_->setValue(data, length);
    if (properties_ & BLECharacteristic::PROPERTY_NOTIFY) {
      characteristic_->notify();
    }
  }
}

} // namespace ble

} // namespace dmc

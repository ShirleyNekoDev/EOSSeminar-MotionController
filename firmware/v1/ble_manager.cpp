#include "ble_manager.h"
#include "ble_abstraction.h"
#include "constants.h"

#define LOG_LOCAL_LEVEL ESP_LOG_DEBUG
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
    // TODO send initial state
  }
  void onDisconnect(BLEServer *server) override {
    clientConnected = false;
    server->startAdvertising();
    // TODO initiate state change somehow
  }
};
}

BLEService *get_dmc_service() { return dmc_service; }

void initialize() {
  esp_log_level_set(TAG, LOG_LOCAL_LEVEL);
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
                                     bool read, bool write, bool notify,
                                     size_t data_size)
    : name_{name} {
  properties_ = 0;
  if (read)
    properties_ |= BLECharacteristic::PROPERTY_READ;
  if (write)
    properties_ |= BLECharacteristic::PROPERTY_WRITE;
  if (notify)
    properties_ |= BLECharacteristic::PROPERTY_NOTIFY;

  characteristic_ = get_dmc_service()->createCharacteristic(uuid, properties_);
  uint8_t empty_val[data_size];
  characteristic_->setValue(empty_val, data_size);
  characteristic_->addDescriptor(new BLE2902());
}

void DMCCharacteristic::write(uint8_t *data, size_t length) {
  ESP_LOGD(TAG, "Writing %i bytes to characteristic \"%s\"", length, name_);
  characteristic_->setValue(data, length);
  if (clientConnected && properties_ & BLECharacteristic::PROPERTY_NOTIFY) {
    characteristic_->notify();
  }
}

void DMCCharacteristic::read(uint8_t *data, size_t length) {
  ESP_LOGV(TAG, "Reading %i bytes from characteristic \"%s\"", length, name_);
  if (length > characteristic_->getLength()) {
    ESP_LOGE(TAG, "Read too much from characteristic \"%s\" (actual size %i)",
             name_, characteristic_->getLength());
  }
  // copy byte by byte
  // TODO optimize this or use library
  for (size_t i = 0; i < length; i++) {
    data[i] = characteristic_->getData()[i];
  }
}

} // namespace ble

} // namespace dmc

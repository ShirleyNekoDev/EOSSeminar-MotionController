#ifndef BLE_ABSTRACTION_H__
#define BLE_ABSTRACTION_H__

// NO BLE headers should be included here

#include <stddef.h>
#include <stdint.h>

// fwd declare to hide bluetooth library
class BLECharacteristic;

namespace dmc {
namespace ble {

class DMCCharacteristic {
protected:
  explicit DMCCharacteristic(const char *name, const char *uuid, bool read,
                             bool write, bool notify, size_t data_size);

  void write(uint8_t *data, size_t length);
  void read(uint8_t *data, size_t length);

private:
  BLECharacteristic *characteristic_;
  uint32_t properties_;
  const char *name_;
};

} // namespace ble
} // namespace dmc

#endif

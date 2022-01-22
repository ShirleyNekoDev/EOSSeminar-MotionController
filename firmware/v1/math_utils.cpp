#include <stdint.h>

#include "math_utils.h"

uint16_t pack_float(float value) {
  uint32_t x = *((uint32_t*)&value);
  return ((x>>16)&0x8000)|((((x&0x7f800000)-0x38000000)>>13)&0x7c00)|((x>>13)&0x03ff);
}


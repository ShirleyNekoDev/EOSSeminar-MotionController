#include <cstdint>

uint16_t pack_float(float value) {
  uint32_t x = *((uint32_t*)&value);
  return ((x>>16)&0x8000)|((((x&0x7f800000)-0x38000000)>>13)&0x7c00)|((x>>13)&0x03ff);
}

struct Vec3 { // 3*2 Byte = 6 Byte
  uint16_t x,y,z;
};

struct Vec2 { // 2*2 Byte = 4 Byte
  uint16_t x,y;
};

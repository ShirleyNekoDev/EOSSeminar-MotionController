#ifndef MATH_UTILS_H__
#define MATH_UTILS_H__

#include <cstdint>

uint16_t pack_float(float value);

struct Vec3 { // 3*2 Byte = 6 Byte
  uint16_t x, y, z;
};

struct Vec2 { // 2*2 Byte = 4 Byte
  uint16_t x, y;
};


#endif



#include <stdint.h>

#include "math_utils.h"

uint16_t pack_float(float value) {
  return (value + 1) * 0.5 * ((1u << 16) - 1);
}


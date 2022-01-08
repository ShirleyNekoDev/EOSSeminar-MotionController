#define LOG_LOCAL_LEVEL ESP_LOG_VERBOSE
#include "esp_log.h"

#include "version.h"
#include <Arduino.h>

static const char *TAG = "MAIN";

void setup() {
  // Set log level
  esp_log_level_set(TAG, LOG_LOCAL_LEVEL);

  ESP_LOGI(TAG, "Starting up firmware v%i.%i...", VERSION_MAJOR, VERSION_MINOR);

  ESP_LOGI(TAG, "Setup complete. Now entering Arduino Core loop.");
}

void loop() {

}

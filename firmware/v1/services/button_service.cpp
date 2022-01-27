#include "services/button_service.h"

#include "board_definitions.h"
#include "math_utils.h"
#include <Bounce2.h>

#include "esp_log.h"

static const char *TAG = "BUTTON_SERVICE";

namespace dmc {

namespace button {

namespace {

Bounce2::Button button_a;
Bounce2::Button button_b;
Bounce2::Button button_menu;

} // namespace

void start() {
  button_a.attach(PIN_BUTTON_A, INPUT_PULLUP);
  button_b.attach(PIN_BUTTON_B, INPUT_PULLUP);
  button_menu.attach(PIN_BUTTON_MENU, INPUT_PULLUP);
}

void refresh() {
  button_a.update();
  button_b.update();
  button_menu.update();
}
bool read_status(Status &button_status) {
  bool update_occurred = false;
  if (button_status.a != button_a.isPressed()) {
    ESP_LOGD(TAG, "The state of button A has changed.");
    update_occurred = true;
    button_status.a = button_a.isPressed();
  }

  if (button_status.b != button_b.isPressed()) {
    ESP_LOGD(TAG, "The state of button B has changed.");
    update_occurred = true;
    button_status.b = button_b.isPressed();
  }

  if (button_status.menu != button_menu.isPressed()) {
    ESP_LOGD(TAG, "The state of the MENU button has changed.");
    update_occurred = true;
    button_status.menu = button_menu.isPressed();
  }

  if (update_occurred) {
    ESP_LOGD(TAG, "An update occured.");
  }

  return update_occurred;
}

} // namespace button

} // namespace dmc

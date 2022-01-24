#include "services/button_service.h"
#include "board_definitions.h"
#include "math_utils.h"
#include <Bounce2.h>

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
  // TODO do important stuff
  button_a.update();
  button_b.update();
  button_menu.update();
}
bool read_status(Status &button_status) {
  bool update_occurred = false;
  if (button_status.a != button_a.isPressed()) {
    update_occurred = true;
  }

  if (button_status.b != button_b.isPressed()) {
    update_occurred = true;
  }

  if (button_status.menu != button_menu.isPressed()) {
    update_occurred = true;
  }

  return update_occurred;
}

} // namespace button

} // namespace dmc

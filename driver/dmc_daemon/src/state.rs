use dmc::ClientUpdate;

#[derive(Clone, Copy, PartialEq, Debug)]
enum ButtonState {
    UP,
    DOWN,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ControllerState {
    battery_level: u8,
    joystick_state: JoystickState,
    button_a_state: ButtonState,
    button_b_state: ButtonState,
    button_menu_state: ButtonState,
}

impl ControllerState {
    pub fn dbg_set_joystick(&mut self, angle: f32, amplitude: f32) -> ClientUpdate {
        assert!(
            0.0 <= amplitude && amplitude <= 1.0,
            "amplitude must be in range 0..1"
        );

        self.joystick_state.x = angle.cos() * amplitude;
        self.joystick_state.y = angle.sin() * amplitude;
        self.joystick_state.into()
    }

    // Returns whether or not the state was updated.
    pub fn button_a_state_transition(&mut self, new_value: bool) -> Option<ClientUpdate> {
        if new_value ^ (self.button_a_state == ButtonState::DOWN) {
            // If the value changed.
            if new_value {
                self.button_a_state = ButtonState::DOWN;
                return Some(ClientUpdate::ButtonADown);
            } else {
                self.button_a_state = ButtonState::UP;
                return Some(ClientUpdate::ButtonAUp);
            }
        }
        // If the value did not change.
        None
    }

    // Returns whether or not the state was updated.
    pub fn button_b_state_transition(&mut self, new_value: bool) -> Option<ClientUpdate> {
        if new_value ^ (self.button_b_state == ButtonState::DOWN) {
            // If the value changed.
            if new_value {
                self.button_b_state = ButtonState::DOWN;
                return Some(ClientUpdate::ButtonBDown);
            } else {
                self.button_b_state = ButtonState::UP;
                return Some(ClientUpdate::ButtonBUp);
            }
        }
        // If the value did not change.
        None
    }

    // Returns whether or not the state was updated.
    pub fn button_menu_state_transition(&mut self, new_value: bool) -> Option<ClientUpdate> {
        if new_value ^ (self.button_menu_state == ButtonState::DOWN) {
            // If the value changed.
            if new_value {
                self.button_menu_state = ButtonState::DOWN;
                return Some(ClientUpdate::ButtonMenuDown);
            } else {
                self.button_menu_state = ButtonState::UP;
                return Some(ClientUpdate::ButtonMenuUp);
            }
        }
        // If the value did not change.
        None
    }
}

impl ControllerState {
    pub fn new() -> Self {
        ControllerState {
            battery_level: 255,
            joystick_state: JoystickState::new(),
            button_a_state: ButtonState::UP,
            button_b_state: ButtonState::UP,
            button_menu_state: ButtonState::UP,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct JoystickState {
    x: f32,
    y: f32,
}

fn unpack_float(packed_f16: u16) -> f32 {
    let h: u32 = packed_f16.into();
    f32::from_bits(((h & 0x8000) << 16) | (((h & 0x7c00) + 0x1C000) << 13) | ((h & 0x03FF) << 13))
}

impl JoystickState {
    fn new() -> Self {
        JoystickState { x: 0.0, y: 0.0 }
    }
    pub fn from_raw_f16s(raw_x: u16, raw_y: u16) -> Self {
        JoystickState {
            x: unpack_float(raw_x),
            y: unpack_float(raw_y),
        }
    }

    fn clamped(&mut self) -> Self {
        JoystickState {
            x: self.x.clamp(-1.0, 1.0),
            y: self.y.clamp(-1.0, 1.0),
        }
    }
}

impl Into<ClientUpdate> for JoystickState {
    fn into(self) -> ClientUpdate {
        ClientUpdate::JoystickMoved {
            x: self.x,
            y: self.y,
        }
    }
}

// This function reads the classic control characteristic's raw data and updates the controller
// state.
pub fn build_classic_control_updates(
    controller_state: &mut ControllerState,
    value: &Vec<u8>,
) -> Option<Vec<ClientUpdate>> {
    assert!(
        value.len() == 5,
        "ClassicControlsCharacteristic's value must be 5 bytes."
    );

    let raw_x: u16 = u16::from_le_bytes(value.as_slice()[0..2].try_into().unwrap());
    let raw_y: u16 = u16::from_le_bytes(value.as_slice()[2..4].try_into().unwrap());

    let button_byte = value[4];
    let button_a = (button_byte & 1u8 << 0) == 0;
    let button_b = (button_byte & 1u8 << 1) == 0;
    let button_menu = (button_byte & 1u8 << 2) == 0;

    let mut updates: Vec<ClientUpdate> = Vec::new();
    {
        // TODO: temporary - remove
        let update: ClientUpdate = ClientUpdate::BatteryStatusChanged { charge: value[0] };
        updates.push(update);
    }

    // Update joystick data
    let joystick_data_read = JoystickState::from_raw_f16s(raw_x, raw_y).clamped();
    if joystick_data_read != controller_state.joystick_state {
        controller_state.joystick_state = joystick_data_read;
        let update: ClientUpdate = controller_state.joystick_state.into();
        updates.push(update);
    }

    // Update button data
    if let Some(update) = controller_state.button_a_state_transition(button_a) {
        updates.push(update);
    }
    if let Some(update) = controller_state.button_b_state_transition(button_b) {
        updates.push(update);
    }
    if let Some(update) = controller_state.button_menu_state_transition(button_menu) {
        updates.push(update);
    }

    if updates.is_empty() {
        None
    } else {
        Some(updates)
    }
}

use dmc::ClientUpdate;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ButtonState {
    UP,
    DOWN,
}

impl Into<ButtonState> for bool {
    fn into(self) -> ButtonState {
        match self {
            true => ButtonState::DOWN,
            false => ButtonState::UP,
        }
    }
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

    pub fn update_buttons(
        &mut self,
        button_a: ButtonState,
        button_b: ButtonState,
        button_menu: ButtonState,
    ) -> Vec<ClientUpdate> {
        let mut updates: Vec<ClientUpdate> = Vec::new();

        if let Some(update) = self.button_a_state_transition(button_a) {
            updates.push(update);
        }
        if let Some(update) = self.button_b_state_transition(button_b) {
            updates.push(update);
        }
        if let Some(update) = self.button_menu_state_transition(button_menu) {
            updates.push(update);
        }

        updates
    }

    fn button_a_state_transition(&mut self, new_value: ButtonState) -> Option<ClientUpdate> {
        if self.button_a_state != new_value {
            self.button_a_state = new_value;
            match new_value {
                ButtonState::DOWN => Some(ClientUpdate::ButtonADown),
                ButtonState::UP => Some(ClientUpdate::ButtonAUp),
            }
        } else {
            None
        }
    }

    fn button_b_state_transition(&mut self, new_value: ButtonState) -> Option<ClientUpdate> {
        if self.button_b_state != new_value {
            self.button_b_state = new_value;
            match new_value {
                ButtonState::DOWN => Some(ClientUpdate::ButtonBDown),
                ButtonState::UP => Some(ClientUpdate::ButtonBUp),
            }
        } else {
            None
        }
    }

    fn button_menu_state_transition(&mut self, new_value: ButtonState) -> Option<ClientUpdate> {
        if self.button_menu_state != new_value {
            self.button_menu_state = new_value;
            match new_value {
                ButtonState::DOWN => Some(ClientUpdate::ButtonMenuDown),
                ButtonState::UP => Some(ClientUpdate::ButtonMenuUp),
            }
        } else {
            None
        }
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
    (packed_f16 as f32 / (((1 << 16) - 1) as f32) * 2.0) - 1.0
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
        value.len() == 6,
        "ClassicControlsCharacteristic's value must be 6 bytes."
    );

    let raw_x: u16 = u16::from_le_bytes(value.as_slice()[0..2].try_into().unwrap());
    let raw_y: u16 = u16::from_le_bytes(value.as_slice()[2..4].try_into().unwrap());

    let button_byte = value[4];
    let button_a = (button_byte & 1u8 << 0) == 0;
    let button_b = (button_byte & 1u8 << 1) == 0;
    let button_menu = (button_byte & 1u8 << 2) == 0;

    let mut updates: Vec<ClientUpdate> = Vec::new();
    // {
    //     // TODO: temporary - remove
    //     let update: ClientUpdate = ClientUpdate::BatteryStatusChanged { charge: value[0] };
    //     updates.push(update);
    // }

    // Update joystick data
    let joystick_data_read = JoystickState::from_raw_f16s(raw_x, raw_y).clamped();
    if joystick_data_read != controller_state.joystick_state {
        controller_state.joystick_state = joystick_data_read;
        let update: ClientUpdate = controller_state.joystick_state.into();
        updates.push(update);
    }

    // Update button data
    updates.append(&mut controller_state.update_buttons(
        button_a.into(),
        button_b.into(),
        button_menu.into(),
    ));

    if updates.is_empty() {
        None
    } else {
        Some(updates)
    }
}

# DIY ~~VR-Motion~~ Controller

Building a realtime wireless BLE input device for the Embedded Operating Systems Seminar 2021 @ HPI Potsdam.

![Final Prototype](https://github.com/ShirleyNekoDev/EOSSeminar-MotionController/blob/main/documentation/images/final-prototype.png?raw=true)

## Contributors

- [@EightSQ](https://github.com/EightSQ)
- [@ShirleyNekoDev](https://github.com/ShirleyNekoDev)

## Targeted features

- 6 DOF (3D rotation and acceleration) tracking
- positional tracking relative to the user's shoulders
- button, trigger and thumbstick
- data transmission via Bluetooth Low-Energy
- battery-powered + USB-C charging
- (Windows only) XBox360 controller emulation

### Real-Time Additional Requirements

- \>100Hz polling rate (10ms time between each sent packet)
- <15ms average latency

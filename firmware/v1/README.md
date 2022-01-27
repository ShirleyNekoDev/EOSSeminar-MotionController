# Device detection


# BLE Services & Characteristics

// https://www.novelbits.io/bluetooth-gatt-services-characteristics/

Bluetooth GATT (Generic Attribute Profile)
- GATT is used exclusively after a connection has been established between the two devices
- Attribute is the generic term for any type of data exposed by the server
    - Attribute type (Universally Unique Identifier or UUID)
    - Attribute Handle
    - Attribute Permissions
- Services and characteristics are types of attributes that serve a specific purpose. Characteristics are the lowest level attribute within a database of attributes
- Service is a grouping of one or more attributes, some of which are characteristics
    - together related attributes that satisfy a specific functionality on the server
- Characteristic is always part of a service and it represents a piece of information/data that a server wants to expose to a client
    - contains other attributes that help define the value it holds:
        - Properties: represented by a number of bits and which defines how a characteristic value can be used. Some examples include: read, write, write without response, notify, indicate
        - Descriptors: used to contain related information about the characteristic Value. Some examples include: extended properties, user description, fields used for subscribing to notifications and indications, and a field that defines the presentation of the value such as the format and the unit of the value
- Profiles are concerned with defining the behavior of both the client and server when it comes to services, characteristics and even connections and security requirements
    - Definition of roles and the relationship between the GATT server and the client.
    - Required Services.
    - Service requirements.
    - How the required services and characteristics are used.
    - Details of connection establishment requirements including advertising and connection parameters.
    - Security considerations.

# Data & Data Access

https://www.novelbits.io/uuid-for-custom-services-and-characteristics/

Standard Profiles, Services, and Characteristics:

Make sure to implement the following mandatory service and its characteristic
// Generic Access Profile (GAP) service with UUID: 0x1800 (SIG-adopted service) set by chipset’s SDK
//    Name with UUID 0x2a00 and value: Bluegiga CR Demo.
//    Appearance with UUID 0x2a01 and value 0x4142
https://development.libelium.com/ble-networking-guide/default-profile-on-ble-module
0000180a-0000-1000-8000-00805f9b34fb Device Information
00002a00-0000-1000-8000-00805f9b34fb Device Name
00002a04-0000-1000-8000-00805f9b34fb Manufacturer Name String
chrome://bluetooth-internals/

Battery Service -> Battery Level
00002a19-0000-1000-8000-00805f9b34fb Battery Level

- https://www.bluetooth.com/specifications/specs/
- https://www.bluetooth.com/specifications/assigned-numbers/

- https://github.com/nkolban/esp32-snippets/blob/master/Documentation/BLE%20C%2B%2B%20Guide.pdf
- https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/bluetooth/index.html

- https://github.com/espressif/arduino-esp32/tree/master/libraries/BLE/examples
- https://embeddedcentric.com/lesson-2-ble-profiles-services-characteristics-device-roles-and-network-topology/
- https://devzone.nordicsemi.com/guides/short-range-guides/b/bluetooth-low-energy/posts/ble-services-a-beginners-tutorial
- https://devzone.nordicsemi.com/guides/short-range-guides/b/bluetooth-low-energy/posts/ble-characteristics-a-beginners-tutorial


# Project-Specific Services & Characteristics

- BatteryService

- DIYMotionControllerService (`21aa4893-f368-46b5-977e-cdfc93555aaf`)
    - ClassicControlsCharacteristic (Button and Joystick) (`0385fe9d-56a6-40a4-b055-9b610cfcfe0c`)
    - FeedbackCharacteristic (LED & Rumble) (`2f99ce60-cafe-4e44-9633-5bb3aea5d6c1`)
    - MotionControlsCharacteristic (Orientation and Acceleration) (`e4b0e66b-f9ac-424b-ae72-59f7fbadf871`)

# Pin Mapping for ESP32-C3-32S (Devkit)

- I2C SDA: IO9 (tested)
- I2C SCL: I010 (tested)
- JOYSTICK_X: IO0/ADC1_CH0
- JOYSTICK_Y: IO1/ADC1_CH1
- LED_R: IO3 (fixed)
- LED_G: IO4 (fixed)
- LED_B: IO5 (fixed)
- BUTTON_MENU: IO2 (RTC pin, wake from deep sleep possible)
- BUTTON_A: IO7
- BUTTON_B: IO8
- RUMBLE_CONTROL: IO6


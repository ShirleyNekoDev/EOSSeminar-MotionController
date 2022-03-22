# Driver

The driver is written in Rust and has submodules for each functionality.

## dmc

DMC lib contains shared data structures used for communication between driver components.

## dmc_daemon

The DMC Daemon contains the driver itself, which has to run in the background in user-space to receive data from the controller, handle its state and share it to other applications.

It is split in two sides, which communicate via an event bus.
The first side handles the BLE connection state to the ESP. It receives packets, unpacks them and updates the internal state representation of the controller. If the state is changed, an update event is generated and sent to the event bus. Received command events from the event bus will be packed and sent to the device. This side can also contain some mock generators, which push fake data / events to the event bus to simulate a controller for debug purposes.
The second side receives events from the event bus and communicates them to external applications. The current implementation provides a WebSocket server to which any app can connect on port 9001. This WebSocket can also receive command events (e.g. set LED color), which it forwards to the first side. 

## dmc_vigem_client

The ViGEm Client uses an installed ViGEmBus driver to emulate our controller as an XBox360 controller. It interconnects the ViGEm interface and the DMC Daemon WebSocket.

## test_webclient

The webclient connects to the DMC Daemon WebSocket and visualizes all update events from the controller, displays the internal state and also has controls to generate command events.

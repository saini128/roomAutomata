# Room Automata

## Project Overview
**Room Automata** is an IoT project designed to automate and remotely control a room's electrical appliances. The system is comprised of three core components: a **Go server**, an **ESP32 microcontroller**, and a **mobile application**. This setup allows for seamless control and monitoring of devices like lights and air conditioners through a centralized server.

<br>

## ⚙️ Components

### 1. ESP32
The ESP32 acts as the physical interface to the room's appliances.

- **IR Transmitter**: A primary function of the ESP32 is to transmit infrared signals to control devices like air conditioners.
- **Future Integration**: The device is designed to be easily expandable to control other electricals, such as lights using **Relays**.
- **Connectivity**: The ESP32 maintains a persistent connection to the Go server via **WebSockets**, allowing for real-time command execution and status updates.

### 2. Go Server
The Go server is the central hub of the system, handling communication between the mobile app and the ESP32.

- **WebSockets**: Serves as the real-time communication channel for the ESP32.
- **REST API**: Provides an interface for the mobile application to send commands and retrieve device status.
- **(Not req. in version 1) Database (SQLite)**: A simple SQLite database is used for data persistence, ensuring that device states are not lost upon application restarts. This can be revised in future architectural updates.

### 3. Mobile App
The mobile application is the user's primary interface for interacting with the system.

- **Status View**: Displays the current status of all room electricals.
- **Control**: Allows users to turn lights and other appliances on or off.
- **Temperature Control**: Provides functionality to increase, decrease, or toggle the power of air conditioners.
- **Timer Functionality**: Users can set automated timers for specific actions.

<br>

## 🛠️ Architecture



The mobile app communicates with the Go server via **REST APIs**. The Go server, in turn, forwards commands to the ESP32 through a persistent **WebSocket** connection. The ESP32 receives these commands and executes them on the connected appliances. Any status changes are sent back through the WebSocket to the server and then updated on the mobile app.

<br>



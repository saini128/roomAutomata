/* ESP32 WiFi Scanning example */

#include "WiFi.h"

API_URL="https://roomAutomata.singhropar.com"
// IR transmitter pins
// Relay light pins

void setup() {
  Serial.begin(115200);
  Serial.println("Initializing WiFi...");
  WiFi.mode(WIFI_STA);
  Serial.println("Setup done!");

  // connect to wifi
  //connect to websocket as maintain as global variable
}


// steps
// 1. always read web sockts for info
// 2. websockets will send if to turn on/ off ac or change temperature
// 3. websockets can also send info to turn on or off lights
void loop() {

}

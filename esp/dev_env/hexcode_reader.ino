#include <Arduino.h>
#include <IRrecv.h>
#include <IRutils.h>

#define RECV_PIN 15   // GPIO pin where IR receiver OUT is connected

IRrecv irrecv(RECV_PIN);
decode_results results;

void setup() {
  Serial.begin(115200);
  irrecv.enableIRIn();   // Start the receiver
  Serial.println("ESP32 IR Receiver Ready...");
}

void loop() {
  if (irrecv.decode(&results)) {
    // Print basic details
    Serial.println(resultToHumanReadableBasic(&results));

    // Print in HEX (commonly used for button matching)
    Serial.print("HEX: 0x");
    Serial.println(results.value, HEX);

    irrecv.resume();  // Receive the next value
  }
}


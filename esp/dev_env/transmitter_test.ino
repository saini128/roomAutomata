#include <Arduino.h>
#include <IRsend.h>

#define IR_SEND_PIN 4   // GPIO for IR LED (use a transistor driver for better range)

IRsend irsend(IR_SEND_PIN);

// Store your IR HEX codes here (replace with your actual remote values)
uint64_t ONoff    = 0x20DF10EF;  // Example: Power button
uint64_t TEMPup   = 0x20DFC03F;  // Example: Volume Up
uint64_t TEMPdown = 0x20DF40BF;  // Example: Volume Down

void setup() {
  Serial.begin(115200);
  irsend.begin();
  Serial.println("Send IR by typing q, w, e in Serial Monitor:");
}

void loop() {
  if (Serial.available()) {
    char cmd = Serial.read();

    if (cmd == 'q') {
      Serial.println("Sending ON/OFF...");
      irsend.sendNEC(ONoff, 32);   // NEC protocol, 32 bits
    } 
    else if (cmd == 'w') {
      Serial.println("Sending TEMP UP...");
      irsend.sendNEC(TEMPup, 32);
    } 
    else if (cmd == 'e') {
      Serial.println("Sending TEMP DOWN...");
      irsend.sendNEC(TEMPdown, 32);
    }
  }
}


#define TRIG_PIN 5
#define ECHO_PIN 18
#define RELAY_PIN 23

long duration;
int distance;
int threshold = 60; // cm - change as needed

void setup() {
  Serial.begin(115200);
  pinMode(TRIG_PIN, OUTPUT);
  pinMode(ECHO_PIN, INPUT);
  pinMode(RELAY_PIN, OUTPUT);

  digitalWrite(RELAY_PIN, LOW); // start with light OFF
}

void loop() {
  // Send ultrasonic pulse
  digitalWrite(TRIG_PIN, LOW);
  delayMicroseconds(2);
  digitalWrite(TRIG_PIN, HIGH);
  delayMicroseconds(10);
  digitalWrite(TRIG_PIN, LOW);

  // Measure echo
  duration = pulseIn(ECHO_PIN, HIGH);
  distance = duration * 0.034 / 2; // cm

  Serial.print("Distance: ");
  Serial.print(distance);
  Serial.println(" cm");

  // Control relay
  if (distance > 0 && distance < threshold) {
    digitalWrite(RELAY_PIN, HIGH);  // turn ON light
  } else {
    digitalWrite(RELAY_PIN, LOW);   // turn OFF light
  }

  delay(500); // small delay
}

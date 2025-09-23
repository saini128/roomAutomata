#include <WiFi.h>
#include <ArduinoJson.h>

// WiFi credentials
const char* ssid = "YOUR_SSID";
const char* password = "YOUR_PASSWORD";

// SSE server details
const char* host = "192.168.0.100";
const int port = 80; // change to 443 if HTTPS (requires WiFiClientSecure)
const char* path = "/ping";

// Device pins
const int RELAY_PIN = 5;
const int AC_PIN = 18;
const int LIGHT_PIN = 19;

WiFiClient client;

void setup() {
  Serial.begin(115200);

  pinMode(RELAY_PIN, OUTPUT);
  pinMode(AC_PIN, OUTPUT);
  pinMode(LIGHT_PIN, OUTPUT);

  connectWiFi();
}

void loop() {
  if (!client.connected()) {
    connectSSE();
  }

  // Read SSE lines
  while (client.connected() && client.available()) {
    String line = client.readStringUntil('\n');
    line.trim();

    if (line.length() == 0) continue;       // skip empty lines
    if (!line.startsWith("data:")) continue; // skip non-data lines

    String jsonStr = line.substring(5); // remove "data:"
    processJSON(jsonStr);
  }

  delay(10); // small delay to avoid blocking
}

// ---------------- WiFi ----------------
void connectWiFi() {
  Serial.print("Connecting to WiFi...");
  WiFi.mode(WIFI_STA);
  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println("\nWiFi connected!");
  Serial.print("IP Address: ");
  Serial.println(WiFi.localIP());
}

// ---------------- SSE connection ----------------
void connectSSE() {
  Serial.printf("Connecting to SSE %s:%d%s\n", host, port, path);

  if (client.connect(host, port)) {
    client.print(String("GET ") + path + " HTTP/1.1\r\n" +
                 "Host: " + host + "\r\n" +
                 "Accept: text/event-stream\r\n" +
                 "Connection: keep-alive\r\n\r\n");
    Serial.println("Connected to SSE server!");
  } else {
    Serial.println("SSE connection failed, retrying in 5 seconds...");
    delay(5000);
  }
}

// ---------------- Parse JSON ----------------
void processJSON(const String& jsonStr) {
  StaticJsonDocument<1024> doc;
  DeserializationError error = deserializeJson(doc, jsonStr);

  if (error) {
    Serial.print("JSON parse error: ");
    Serial.println(error.c_str());
    return;
  }

  JsonArray devices = doc["devices"].as<JsonArray>();
  for (JsonObject device : devices) {
    String type = device["type"].as<String>();

    // currently handling only switch for simplicity
    // if (type == "ac") {
    //   handleAC(device["state"]);
    // } else if (type == "light") {
    //   handleLight(device["state"]);
    // } else 
    if (type == "switch") {
      handleSwitch(device["state"]);
    }
  }
}

// ---------------- Device handlers ----------------
void handleAC(JsonObject state) {
  bool on = state["on"] | false;
  int temp = state["temperature"] | 24;

  Serial.printf("AC state: on=%d, temp=%d\n", on, temp);
  digitalWrite(AC_PIN, on ? HIGH : LOW);

  // TODO: Send IR signal to set temperature if needed
}

void handleLight(JsonVariant state) {
  String lightState = state.as<String>();
  Serial.printf("Light state: %s\n", lightState.c_str());

  if (lightState == "on") {
    digitalWrite(LIGHT_PIN, HIGH);
  } else if (lightState == "off") {
    digitalWrite(LIGHT_PIN, LOW);
  } else if (lightState == "auto") {
    // implement auto behavior
    digitalWrite(LIGHT_PIN, LOW);
  }
}

void handleSwitch(JsonVariant state) {
  String switchState = state.as<String>();
  Serial.printf("Switch state: %s\n", switchState.c_str());

  if (switchState == "on") {
    digitalWrite(RELAY_PIN, HIGH);
  } else {
    digitalWrite(RELAY_PIN, LOW);
  }
}

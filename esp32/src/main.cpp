#include <Arduino.h>
#include <constants.h>
#include <PubSubClient.h>
#include <utils.h>
#include <WiFiNINA.h>

uint64_t prev_time = 0;
uint64_t prev_sec = 0;
uint16_t led_frequency = 10;
WiFiClient wifiClient;
PubSubClient mqttClient(wifiClient);

void setup() {
  Serial.begin(115200);

	ledcAttachPin(PIN_LED_BUTILIN, LED_CHANNEL);
	ledcSetup(LED_CHANNEL, 5000, 8);

	delay(500);
	wifi_setup();
	delay(500);
	mqtt_setup(mqttClient);
	delay(500);
	ledcWrite(LED_CHANNEL, led_frequency);
}

void loop() {
}
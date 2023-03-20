#include <Arduino.h>
#include <constants.h>
#include <PubSubClient.h>
#include <utils.hpp>
#include <WiFi.h>

uint64_t prev_time = 0;
uint64_t prev_sec = 0;
WiFiClient wifi_client;
PubSubClient mqtt_client(wifi_client);
/// @brief the 1st bit is 1 if AC is on. The 2nd bit is 1 if light is on.
uint8_t devices_state = 0b00000000;
// bool is_ac_on = false;
// bool is_light_on = false;
uint8_t led_frequency = 0;

void setup() {
	Serial.begin(115200);

	ledcAttachPin(PIN_LED_BUTILIN, LED_CHANNEL);
	ledcSetup(LED_CHANNEL, 5000, 8);

	delay(500);
	wifi_setup();
	delay(500);
	mqtt_setup(mqtt_client);
	delay(500);
	ledcWrite(LED_CHANNEL, led_frequency);
}

void loop() {
	if (!mqtt_client.connected()) {
		mqtt_reconnect(mqtt_client);
	}
	mqtt_client.loop();
}

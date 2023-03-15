#include <Arduino.h>
#include <constants.h>
#include <PubSubClient.h>
#include <utils.h>
#include <WiFi.h>

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
	if (!mqttClient.connected()) {
		mqtt_reconnect(mqttClient);
	}
	mqttClient.loop();

	uint64_t now = millis();
	uint64_t now_sec = now / 1000;
	if (now - prev_time < 100) {
		return;
	}
	prev_time = now;

	int button_state = digitalRead(PIN_BUTTON);
	if (button_state == LOW) {
		handle_button_pressed(&led_frequency);
		Serial.printf("LED freq: %d\r\n", led_frequency);
	}

	if (prev_sec != now_sec && now_sec % 5 == 0) {
		Serial.printf("%d    |    ", now);
		if (mqtt_publish(mqttClient, led_frequency)) {
			Serial.println("Published data");
		} else {
			Serial.println("Can't publish data");
		}
	}

	prev_sec = now_sec;
}

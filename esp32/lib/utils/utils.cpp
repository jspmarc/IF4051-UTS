//
// Created by josep on 21/02/23.
//

#include "utils.h"
#include "../../include/constants.h"
#include <Arduino.h>
#include <WiFi.h>

void wifi_setup() {
	Serial.print("Setting up Wi-Fi.");

	WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

	while (WiFi.status() != WL_CONNECTED) {
		delay(500);
		Serial.print(".");
	}

	Serial.println();
	Serial.print("Wi-Fi connected, address: ");
	Serial.println(WiFi.localIP());
}

void mqtt_setup(PubSubClient &client) {
	client.setServer(MQTT_HOST, MQTT_PORT);
	client.setCallback(__mqtt_callback);
}

void mqtt_reconnect(PubSubClient &client) {
	while (!client.connected()) {
		Serial.println("Connecting to MQTT broker...");
		if (client.connect(MQTT_ID)) {
			client.subscribe(MQTT_IN_TOPIC);
			Serial.printf("MQTT connected and subscribed to %s\r\n", MQTT_IN_TOPIC);
		} else {
			Serial.printf("Failed to connect, rc=%d\r\n", client.state());
			Serial.println("Retrying in 5 secs...");
			delay(5000);
		}
	}
}

bool mqtt_publish(PubSubClient &client, uint8_t led_frequency) {
	char payload[15];
	sprintf(payload, "13519164:%d", led_frequency);
	return client.publish(MQTT_OUT_TOPIC, payload);
}

void __mqtt_callback(char *topic, uint8_t *payload, unsigned int length) {
	Serial.printf("Message from %s: ", topic);
	for (int i = 0; i < length; i++) {
		delay(2000);
		Serial.print((char) payload[i]);
	}
	Serial.println();
}

void handle_button_pressed(uint16_t *led_frequency) {
	if (*led_frequency >= 255) {
		*led_frequency = -1;
	}

	(*led_frequency)++;
	ledcWrite(LED_CHANNEL, *led_frequency);
}
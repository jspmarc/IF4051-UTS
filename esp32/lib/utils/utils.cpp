//
// Created by josep on 21/02/23.
//

#include "utils.hpp"
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
			client.subscribe(MQTT_IN_DEVICE_TOPIC);
			client.subscribe(MQTT_IN_PING_TOPIC);
			Serial.printf(
				"MQTT connected and subscribed to [%s] and [%s]\r\n",
				MQTT_IN_DEVICE_TOPIC,
				MQTT_IN_PING_TOPIC
			);
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
	return client.publish(MQTT_OUT_DEVICE_TOPIC, payload);
}

static void __mqtt_callback(char *topic, uint8_t *payload, unsigned int length) {
	extern PubSubClient mqtt_client;

	Serial.printf("Message from topic: %s | with payload length: %u\r\n", topic, length);

	if (strcmp(MQTT_IN_DEVICE_TOPIC, topic) == 0 && length == 1) {
		extern uint8_t devices_state;
		extern uint8_t led_frequency;

		devices_state = *payload;
		switch (devices_state)
		{
		case 0:
			led_frequency = 0;
			break;
		case 1:
			led_frequency = 63;
			break;
		case 2:
			led_frequency = 127;
			break;
		default:
			led_frequency = 255;
		}

		ledcWrite(LED_CHANNEL, led_frequency);
	} else if (strcmp(MQTT_IN_PING_TOPIC, topic) == 0) {
		mqtt_client.publish(MQTT_OUT_PONG_TOPIC, "pong");
	}
}

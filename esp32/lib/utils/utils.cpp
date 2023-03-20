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
			client.subscribe(MQTT_IN_AC_TOPIC);
			client.subscribe(MQTT_IN_LIGHT_TOPIC);
			client.subscribe(MQTT_IN_RESET_TOPIC);
			client.subscribe(MQTT_IN_PING_TOPIC);
			Serial.printf(
				"MQTT connected and subscribed to [%s], [%s], [%s], and [%s]\r\n",
				MQTT_IN_AC_TOPIC,
				MQTT_IN_LIGHT_TOPIC,
				MQTT_IN_RESET_TOPIC,
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
	extern uint8_t devices_state;
	extern uint8_t led_frequency;

	bool update_frequency = false;

	Serial.printf("Message from topic: %s | with payload length: %u\r\n", topic, length);

	if (strcmp(MQTT_IN_PING_TOPIC, topic) == 0) {
		mqtt_client.publish(MQTT_OUT_PONG_TOPIC, "pong");
		return;
	}

	if (strcmp(MQTT_IN_RESET_TOPIC, topic) == 0) {
		devices_state = 0;
		led_frequency = 0;
		ledcWrite(LED_CHANNEL, led_frequency);
	}

	Serial.printf("Before: %d\r\n", devices_state);
	if (strcmp(MQTT_IN_AC_TOPIC, topic) == 0 && length == 1) {
		int request = *payload & 1;
		if (request == 1) {
			// turns on AC
			devices_state |= 0b00000001;
		} else {
			// turns off AC
			devices_state &= 0b00000010;
		}
		update_frequency = true;
	} else if (strcmp(MQTT_IN_LIGHT_TOPIC, topic) == 0 && length == 1) {
		int request = *payload & 1;
		if (request == 1) {
			// turns on light
			devices_state |= 0b00000010;
		} else {
			// turns off light
			devices_state &= 0b00000001;
		}
		update_frequency = true;
	}
	Serial.printf("After: %d\r\n", devices_state);

	if (update_frequency) {
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
	}
}

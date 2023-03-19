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
			Serial.printf("MQTT connected and subscribed to %s\r\n", MQTT_IN_DEVICE_TOPIC, MQTT_IN_PING_TOPIC);
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

	char *payload_str = (char *)malloc(sizeof(char) * (length + 1));
	memcpy(payload_str, payload, length);
	payload_str[length] = '\0';
	if (strcmp(MQTT_IN_DEVICE_TOPIC, topic) == 0) {
		Serial.printf("Message for device with payload: %s\r\n", payload_str);
	} else if (strcmp(MQTT_IN_PING_TOPIC, topic) == 0) {
		Serial.printf("Message for ping with payload: %s\r\n", payload_str);
		mqtt_client.publish(MQTT_OUT_PONG_TOPIC, "pong");
	}
	free(payload_str);
}

void handle_button_pressed(uint16_t *led_frequency) {
	if (*led_frequency >= 255) {
		*led_frequency = -1;
	}

	(*led_frequency)++;
	ledcWrite(LED_CHANNEL, *led_frequency);
}
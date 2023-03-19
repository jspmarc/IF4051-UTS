//
// Created by josep on 21/02/23.
//

#ifndef UTILS_HPP
#define UTILS_HPP

#include <PubSubClient.h>
#include <WiFi.h>

void wifi_setup();

void mqtt_setup(PubSubClient &client);
void mqtt_reconnect(PubSubClient &client);
bool mqtt_publish(PubSubClient &client, uint8_t led_frequency);
void __mqtt_callback(char *topic, uint8_t *payload, unsigned int length);

void handle_button_pressed(uint16_t *led_frequency);

#endif//UTILS_HPP

//
// Created by josep on 21/02/23.
//

#ifndef CONSTANTS_H
#define CONSTANTS_H

#include <cstdint>
#include "constants_secret.h"

const uint8_t PIN_LED_BUTILIN = 2;
const uint8_t PIN_BUTTON = 0;
const uint8_t LED_CHANNEL = 0;

const uint8_t AC_LED_FREQUENCY = 15;
const uint8_t LIGHT_LED_FREQUENCY = 127;

#define MQTT_ID "esp32"
#define MQTT_IN_DEVICE_TOPIC "device"
#define MQTT_IN_PING_TOPIC "ping"
#define MQTT_OUT_DEVICE_TOPIC "device-response"
#define MQTT_OUT_PONG_TOPIC "pong"

#endif//CONSTANTS_H

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

#define MQTT_ID "esp32"
#define MQTT_IN_TOPIC "in-tugas"
#define MQTT_OUT_TOPIC "out-tugas"

#endif//CONSTANTS_H

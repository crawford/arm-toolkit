/*
 * Copyright 2015 Alex Crawford
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include <stdlib.h>
#include "gpio_11xx.h"
#include "lamp.h"

static inline void _init(lamp_t lamp);
static inline void _set_on(lamp_t lamp);
static inline void _set_off(lamp_t lamp);
static inline void _toggle(lamp_t lamp);

struct _lamp {
	int port;
	int pin;
};

void lamp_init(lamp_t lamp)
{
	_init(lamp);
	_set_off(lamp);
}

void lamp_set_on(lamp_t lamp)
{
	_set_on(lamp);
}

void lamp_set_off(lamp_t lamp)
{
	_set_off(lamp);
}

void lamp_toggle(lamp_t lamp)
{
	_toggle(lamp);
}

static inline void _init(lamp_t lamp)
{
	Chip_GPIO_Init(LPC_GPIO);
	Chip_GPIO_SetPinDIROutput(LPC_GPIO, lamp->port, lamp->pin);
}

static inline void _set_on(lamp_t lamp)
{
	Chip_GPIO_SetPinOutLow(LPC_GPIO, lamp->port, lamp->pin);
}

static inline void _set_off(lamp_t lamp)
{
	Chip_GPIO_SetPinOutHigh(LPC_GPIO, lamp->port, lamp->pin);
}

static inline void _toggle(lamp_t lamp)
{
	Chip_GPIO_SetPinToggle(LPC_GPIO, lamp->port, lamp->pin);
}


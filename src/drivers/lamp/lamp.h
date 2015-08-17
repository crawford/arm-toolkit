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

#ifndef _LAMP_H_
#define _LAMP_H_

#include <stdbool.h>

struct _lamp;
typedef const struct _lamp *lamp_t;

extern lamp_t g_lamp_fault;
extern lamp_t g_lamp_power;
extern lamp_t g_lamp_conn;

void lamp_init(lamp_t lamp);

void lamp_toggle(lamp_t lamp);

void lamp_set_on(lamp_t lamp);

void lamp_set_off(lamp_t lamp);

static inline void lamp_set(lamp_t lamp,
                            bool on)
{
	if (on)
		return lamp_set_on(lamp);
	else
		return lamp_set_off(lamp);
}

#endif // _LAMP_H_


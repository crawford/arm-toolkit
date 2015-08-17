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

#ifndef _TIMER_H_
#define _TIMER_H_

#include <stdint.h>

#define MSEC_PER_SEC   1000
#define USEC_PER_MSEC  1000
#define USEC_PER_SEC   (MSEC_PER_SEC * USEC_PER_MSEC)

struct _timer;
typedef struct _timer *timer_t;

typedef void (*timer_handler_t)(timer_t source,
                                void *context);

extern timer_t g_timer_16_0;
extern timer_t g_timer_16_1;
extern timer_t g_timer_32_0;
extern timer_t g_timer_32_1;

void timer_init(timer_t timer);

void timer_start(timer_t timer);

void timer_stop(timer_t timer);

void timer_set_period_ms(timer_t timer,
                         uint32_t period_ms);

void timer_set_period_us(timer_t timer,
                         uint32_t period_us);

void timer_register_handler(timer_t timer,
                            timer_handler_t handler,
                            void *context);

#endif // _TIMER_H_


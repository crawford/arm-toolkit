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

#ifndef _ADC_H_
#define _ADC_H_

#include <stdbool.h>

typedef enum {
	ADC_CH_0 = 0,
	ADC_CH_1,
	ADC_CH_2,
	ADC_CH_3,
	ADC_CH_4,
	ADC_CH_5,
	ADC_CH_6,
	ADC_CH_7,

	_ADC_COUNT,
} adc_t;

typedef void (*adc_handler_t)(adc_t channel,
                              void *context);

void adc_init(void);

bool adc_reading(adc_t channel,
                 uint16_t *reading);

void adc_trigger_sample(adc_t channel);

void adc_register_reading_handler(adc_t channel,
                                  adc_handler_t handler,
                                  void *context);

#endif // _ADC_H_


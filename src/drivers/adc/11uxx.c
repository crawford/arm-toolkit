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

#include "gpio_11xx.h"
#include "cmsis.h"
#include "lpc_types.h"
#include "assert.h"
#include "adc.h"

static inline void _isr(void);

static struct {
	adc_handler_t handler;
	void *context;
} _handlers[_ADC_COUNT];

void ADC_IRQHandler(void)
{
	__disable_irq();
	NVIC_ClearPendingIRQ(ADC_IRQn);
	__enable_irq();

	_isr();
}

void adc_init(void)
{
	for (adc_t ch = ADC_CH_0; ch < _ADC_COUNT; ch++)
		_handlers[ch].handler = NULL;

	ADC_CLOCK_SETUP_T clock_setup;
	Chip_ADC_Init(LPC_ADC, &clock_setup);
	Chip_ADC_Int_SetGlobalCmd(LPC_ADC, DISABLE);

	NVIC_ClearPendingIRQ(ADC_IRQn);
	NVIC_EnableIRQ(ADC_IRQn);
}

bool adc_reading(adc_t channel,
                 uint16_t *reading)
{
	ASSERT(channel < _ADC_COUNT);
	ASSERT(reading);

	return (Chip_ADC_ReadValue(LPC_ADC, channel, reading) == SUCCESS);
}

void adc_trigger_sample(adc_t channel)
{
	ASSERT(channel < _ADC_COUNT);

	Chip_ADC_EnableChannel(LPC_ADC, channel, ENABLE);
	Chip_ADC_SetStartMode(LPC_ADC, ADC_START_NOW, ADC_TRIGGERMODE_RISING);
}

void adc_register_reading_handler(adc_t channel,
                                  adc_handler_t handler,
                                  void *context)
{
	ASSERT(channel < _ADC_COUNT);

	_handlers[channel].handler = handler;
	_handlers[channel].context = context;

	/*if (handler)
		Chip_ADC_Int_SetChannelCmd(LPC_ADC, channel, ENABLE);
	else
		Chip_ADC_Int_SetChannelCmd(LPC_ADC, channel, DISABLE);*/
}

static inline void _isr(void)
{
	for (adc_t channel = ADC_CH_0; channel < _ADC_COUNT; channel++)
	{
		if (_handlers[channel].handler &&
                    Chip_ADC_ReadStatus(LPC_ADC, channel, ADC_DR_DONE_STAT))
		{
			_handlers[channel].handler(channel, _handlers[channel].context);
		}
	}
}


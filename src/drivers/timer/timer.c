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

#include "clock_11xx.h"
#include "timer_11xx.h"
#include "timer.h"

#define MATCH_CHANNEL  0

static void _set_period(timer_t timer,
                        uint32_t period,
                        uint32_t timescale);

struct _timer {
	LPC_TIMER_T *const base;
	const IRQn_Type irq;
	timer_handler_t handler;
	void *context;
};

enum timer_ids {
	TIMER_16_0 = 0,
	TIMER_16_1,
	TIMER_32_0,
	TIMER_32_1,

	_TIMER_COUNT,
};

static struct _timer _g_timers[_TIMER_COUNT] = {
	[TIMER_16_0] = {
		.base = LPC_TIMER16_0,
		.irq  = TIMER_16_0_IRQn,
	},
	[TIMER_16_1] = {
		.base = LPC_TIMER16_1,
		.irq  = TIMER_16_1_IRQn,
	},
	[TIMER_32_0] = {
		.base = LPC_TIMER32_0,
		.irq  = TIMER_32_0_IRQn,
	},
	[TIMER_32_1] = {
		.base = LPC_TIMER32_1,
		.irq  = TIMER_32_1_IRQn,
	},
};

timer_t g_timer_16_0 = &_g_timers[TIMER_16_0];
timer_t g_timer_16_1 = &_g_timers[TIMER_16_1];
timer_t g_timer_32_0 = &_g_timers[TIMER_32_0];
timer_t g_timer_32_1 = &_g_timers[TIMER_32_1];

void timer_init(timer_t timer)
{
	Chip_TIMER_Init(timer->base);
}

void timer_start(timer_t timer)
{
	NVIC_ClearPendingIRQ(timer->irq);
	NVIC_EnableIRQ(timer->irq);

	Chip_TIMER_Enable(timer->base);
}

void timer_stop(timer_t timer)
{
	Chip_TIMER_Disable(timer->base);
}

void timer_set_period_ms(timer_t timer,
                         uint32_t period_ms)
{
	_set_period(timer, period_ms, MSEC_PER_SEC);
}

void timer_set_period_us(timer_t timer,
                         uint32_t period_us)
{
	_set_period(timer, period_us, USEC_PER_SEC);
}

void timer_register_handler(timer_t timer,
                            timer_handler_t handler,
                            void *context)
{
	timer->context = context;
	timer->handler = handler;
}

static void _handle_match(timer_t timer)
{
	__disable_irq();
	Chip_TIMER_ClearMatch(timer->base, MATCH_CHANNEL);
	NVIC_ClearPendingIRQ(timer->irq);
	__enable_irq();

	if (timer->handler)
		timer->handler(timer, timer->context);
}

void CT16B0_IRQHandler(void)
{
	_handle_match(g_timer_16_0);
}

void CT16B1_IRQHandler(void)
{
	_handle_match(g_timer_16_1);
}

void CT32B0_IRQHandler(void)
{
	_handle_match(g_timer_32_0);
}

void CT32B1_IRQHandler(void)
{
	_handle_match(g_timer_32_1);
}

static void _set_period(timer_t timer,
                        uint32_t period,
                        uint32_t timescale)
{
	uint32_t pclk_freq = Chip_Clock_GetSystemClockRate();
	uint32_t prescale = (pclk_freq / timescale) - 1;
	Chip_TIMER_PrescaleSet(timer->base, prescale);

	Chip_TIMER_SetMatch(timer->base, MATCH_CHANNEL, period);
	Chip_TIMER_MatchEnableInt(timer->base, MATCH_CHANNEL);
	Chip_TIMER_ResetOnMatchEnable(timer->base, MATCH_CHANNEL);
}


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

	.syntax unified
	.text
	.thumb
	.thumb_func
	.align  2
	.globl  HardFault_Handler
	.type   HardFault_Handler, %function
HardFault_Handler:
	movs    r0,   #4
	mov     r1,   lr
	tst     r0,   r1
	beq     _msp
_psp:
	mrs     r0,   psp
	b       _call_handler
_msp:
	mrs     r0,   msp
_call_handler:
	ldr     r2,   =HardFault_Handler_c
	bx      r2

	.weak    HardFault_Handler_c
	.type    HardFault_Handler_c, %function
HardFault_Handler_c:
	b    .


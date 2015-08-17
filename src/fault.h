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

typedef struct {
	__I uint32_t  R0;
	__I uint32_t  R1;
	__I uint32_t  R2;
	__I uint32_t  R3;
	__I uint32_t  R12;
	__I uint32_t  LR;
	__I uint32_t  PC;
	__I xPSR_Type xPSR;
} stack_regs_t;


/* File: startup_ARMCM0.S
 * Purpose: startup file for Cortex-M0 devices. Should use with
 *   GCC for ARM Embedded Processors
 * Version: V1.3
 * Date: 08 Feb 2012
 *
 * Copyright (c) 2012, ARM Limited
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
    * Neither the name of the ARM Limited nor the
      names of its contributors may be used to endorse or promote products
      derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL ARM LIMITED BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
    .syntax unified
    .arch armv6-m

    .section .stack
    .align 3
#ifdef __STACK_SIZE
    .equ    Stack_Size, __STACK_SIZE
#else
    .equ    Stack_Size, 0x400
#endif
    .globl    __StackTop
    .globl    __StackLimit
__StackLimit:
    .space    Stack_Size
    .size __StackLimit, . - __StackLimit
__StackTop:
    .size __StackTop, . - __StackTop

    .section .heap
    .align 3
#ifdef __HEAP_SIZE
    .equ    Heap_Size, __HEAP_SIZE
#else
    .equ    Heap_Size, 0xC00
#endif
    .globl    __HeapBase
    .globl    __HeapLimit
__HeapBase:
    .if    Heap_Size
    .space    Heap_Size
    .endif
    .size __HeapBase, . - __HeapBase
__HeapLimit:
    .size __HeapLimit, . - __HeapLimit

    .section .isr_vector
    .align 2
    .globl __isr_vector
__isr_vector:
    .long    __StackTop            /* Top of Stack                           */
    .long    Reset_Handler         /* Reset Handler                          */
    .long    NMI_Handler           /* NMI Handler                            */
    .long    HardFault_Handler     /* Hard Fault Handler                     */
    .long    0                     /* Reserved                               */
    .long    0                     /* Reserved                               */
    .long    0                     /* Reserved                               */
    .long    0                     /* Reserved                               */
    .long    0                     /* Reserved                               */
    .long    0                     /* Reserved                               */
    .long    0                     /* Reserved                               */
    .long    SVC_Handler           /* SVCall Handler                         */
    .long    0                     /* Reserved                               */
    .long    0                     /* Reserved                               */
    .long    PendSV_Handler        /* PendSV Handler                         */
    .long    SysTick_Handler       /* SysTick Handler                        */

    /* External interrupts */
    .long    PIN_INT0_IRQHandler   /*  0:  GPIO pin interrupt 0              */
    .long    PIN_INT1_IRQHandler   /*  1:  GPIO pin interrupt 1              */
    .long    PIN_INT2_IRQHandler   /*  2:  GPIO pin interrupt 2              */
    .long    PIN_INT3_IRQHandler   /*  3:  GPIO pin interrupt 3              */
    .long    PIN_INT4_IRQHandler   /*  4:  GPIO pin interrupt 4              */
    .long    PIN_INT5_IRQHandler   /*  5:  GPIO pin interrupt 5              */
    .long    PIN_INT6_IRQHandler   /*  6:  GPIO pin interrupt 6              */
    .long    PIN_INT7_IRQHandler   /*  7:  GPIO pin interrupt 7              */
    .long    GINT0_IRQHandler      /*  8:  GPIO GROUP0 interrupt             */
    .long    GINT1_IRQHandler      /*  9:  GPIO GROUP1 interrupt             */
    .long    0                     /* 10:  Reserved                          */
    .long    0                     /* 11:  Reserved                          */
    .long    0                     /* 12:  Reserved                          */
    .long    0                     /* 13:  Reserved                          */
    .long    SSP1_IRQHandler       /* 14:  SSP1 interrupt                    */
    .long    I2C_IRQHandler        /* 15:  I2C interrupt                     */
    .long    CT16B0_IRQHandler     /* 16:  CT16B0 interrupt                  */
    .long    CT16B1_IRQHandler     /* 17:  CT16B1 interrupt                  */
    .long    CT32B0_IRQHandler     /* 18:  CT32B0 interrupt                  */
    .long    CT32B1_IRQHandler     /* 19:  CT32B1 interrupt                  */
    .long    SSP0_IRQHandler       /* 20:  SSP0 interrupt                    */
    .long    USART_IRQHandler      /* 21:  USART interrupt                   */
    .long    USB_IRQ_IRQHandler    /* 22:  USB_IRQ interrupt                 */
    .long    USB_FIQ_IRQHandler    /* 23:  USB_FIQ interrupt                 */
    .long    ADC_IRQHandler        /* 24:  ADC interrupt                     */
    .long    WWDT_IRQHandler       /* 25:  Windowed Watchdog interrupt       */
    .long    BOD_IRQHandler        /* 26:  Brown-out interrupt               */
    .long    FLASH_IRQHandler      /* 27:  Flash/EEPROM interface interrupt  */
    .long    0                     /* 28:  Reserved                          */
    .long    0                     /* 29:  Reserved                          */
    .long    USB_WAKEUP_IRQHandler /* 30:  USB wake-up interrupt             */
    .long    IOH_IRQHandler        /* 31:  I/O Handler interrupt             */


    .size    __isr_vector, . - __isr_vector

    .text
    .thumb
    .thumb_func
    .align 2
    .globl    Reset_Handler
    .type    Reset_Handler, %function
Reset_Handler:
/*     Loop to copy data from read only memory to RAM. The ranges
 *      of copy from/to are specified by following symbols evaluated in
 *      linker script.
 *      __etext: End of code section, i.e., begin of data sections to copy from.
 *      __data_start__/__data_end__: RAM address range that data should be
 *      copied to. Both must be aligned to 4 bytes boundary.  */

    ldr    r1, =__etext
    ldr    r2, =__data_start__
    ldr    r3, =__data_end__

    subs    r3, r2
    ble    .flash_to_ram_loop_end

    movs    r4, 0
.flash_to_ram_loop:
    ldr    r0, [r1,r4]
    str    r0, [r2,r4]
    adds    r4, 4
    cmp    r4, r3
    blt    .flash_to_ram_loop
.flash_to_ram_loop_end:

#ifndef __NO_SYSTEM_INIT
    ldr    r0, =SystemInit
    blx    r0
#endif

    ldr    r0, =_start
    bx    r0
    .pool
    .size Reset_Handler, . - Reset_Handler

/*    Macro to define default handlers. Default handler
 *    will be weak symbol and just dead loops. They can be
 *    overwritten by other handlers */
    .macro    def_irq_handler    handler_name
    .align 1
    .thumb_func
    .weak    \handler_name
    .type    \handler_name, %function
\handler_name :
    b    .
    .size    \handler_name, . - \handler_name
    .endm

    def_irq_handler    NMI_Handler
    def_irq_handler    HardFault_Handler
    def_irq_handler    SVC_Handler
    def_irq_handler    PendSV_Handler
    def_irq_handler    SysTick_Handler
    def_irq_handler    Default_Handler

    def_irq_handler    PIN_INT0_IRQHandler
    def_irq_handler    PIN_INT1_IRQHandler
    def_irq_handler    PIN_INT2_IRQHandler
    def_irq_handler    PIN_INT3_IRQHandler
    def_irq_handler    PIN_INT4_IRQHandler
    def_irq_handler    PIN_INT5_IRQHandler
    def_irq_handler    PIN_INT6_IRQHandler
    def_irq_handler    PIN_INT7_IRQHandler
    def_irq_handler    GINT0_IRQHandler
    def_irq_handler    GINT1_IRQHandler
    def_irq_handler    SSP1_IRQHandler
    def_irq_handler    I2C_IRQHandler
    def_irq_handler    CT16B0_IRQHandler
    def_irq_handler    CT16B1_IRQHandler
    def_irq_handler    CT32B0_IRQHandler
    def_irq_handler    CT32B1_IRQHandler
    def_irq_handler    SSP0_IRQHandler
    def_irq_handler    USART_IRQHandler
    def_irq_handler    USB_IRQ_IRQHandler
    def_irq_handler    USB_FIQ_IRQHandler
    def_irq_handler    ADC_IRQHandler
    def_irq_handler    WWDT_IRQHandler
    def_irq_handler    BOD_IRQHandler
    def_irq_handler    FLASH_IRQHandler
    def_irq_handler    USB_WAKEUP_IRQHandler
    def_irq_handler    IOH_IRQHandler

    .end

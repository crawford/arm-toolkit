/**************************************************************************//**
 * @file     ARMCM0.h
 * @brief    CMSIS Core Peripheral Access Layer Header File for
 *           ARMCM0 Device Series
 * @version  V1.07
 * @date     30. January 2012
 *
 * @note
 * Copyright (C) 2012 ARM Limited. All rights reserved.
 *
 * @par
 * ARM Limited (ARM) is supplying this software for use with Cortex-M 
 * processor based microcontrollers.  This file can be freely distributed 
 * within development tools that are supporting such ARM based processors. 
 *
 * @par
 * THIS SOFTWARE IS PROVIDED "AS IS".  NO WARRANTIES, WHETHER EXPRESS, IMPLIED
 * OR STATUTORY, INCLUDING, BUT NOT LIMITED TO, IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE APPLY TO THIS SOFTWARE.
 * ARM SHALL NOT, IN ANY CIRCUMSTANCES, BE LIABLE FOR SPECIAL, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, FOR ANY REASON WHATSOEVER.
 *
 ******************************************************************************/

#ifndef ARMCM0_H
#define ARMCM0_H

#ifdef __cplusplus
extern "C" {
#endif


/* -------------------------  Interrupt Number Definition  ------------------------ */

typedef enum IRQn
{
/* -------------------  Cortex-M0 Processor Exceptions Numbers  ------------------- */
  NonMaskableInt_IRQn           = -14,      /*!<  2 Non Maskable Interrupt          */
  HardFault_IRQn                = -13,      /*!<  3 HardFault Interrupt             */



  SVCall_IRQn                   =  -5,      /*!< 11 SV Call Interrupt               */

  PendSV_IRQn                   =  -2,      /*!< 14 Pend SV Interrupt               */
  SysTick_IRQn                  =  -1,      /*!< 15 System Tick Interrupt           */

/* ----------------------  ARMCM0 Specific Interrupt Numbers  --------------------- */
  PIN_INT0_IRQn                 =   0,      /*!< GPIO pin interrupt 0               */
  PIN_INT1_IRQn                 =   1,      /*!< GPIO pin interrupt 1               */
  PIN_INT2_IRQn                 =   2,      /*!< GPIO pin interrupt 2               */
  PIN_INT3_IRQn                 =   3,      /*!< GPIO pin interrupt 3               */
  PIN_INT4_IRQn                 =   4,      /*!< GPIO pin interrupt 4               */
  PIN_INT5_IRQn                 =   5,      /*!< GPIO pin interrupt 5               */
  PIN_INT6_IRQn                 =   6,      /*!< GPIO pin interrupt 6               */
  PIN_INT7_IRQn                 =   7,      /*!< GPIO pin interrupt 7               */
  GINT0_IRQn                    =   8,      /*!< GPIO GROUP0 interrupt              */
  GINT1_IRQn                    =   9,      /*!< GPIO GROUP1 interrupt              */
  SSP1_IRQn                     =  14,      /*!< SSP1 interrupt                     */
  I2C_IRQn                      =  15,      /*!< I2C interrupt                      */
  CT16B0_IRQn                   =  16,      /*!< CT16B0 interrupt                   */
  CT16B1_IRQn                   =  17,      /*!< CT16B1 interrupt                   */
  CT32B0_IRQn                   =  18,      /*!< CT32B0 interrupt                   */
  CT32B1_IRQn                   =  19,      /*!< CT32B1 interrupt                   */
  SSP0_IRQn                     =  20,      /*!< SSP0 interrupt                     */
  USART_IRQn                    =  21,      /*!< USART interrupt                    */
  USB_IRQ_IRQn                  =  22,      /*!< USB_IRQ interrupt                  */
  USB_FIQ_IRQn                  =  23,      /*!< USB_FIQ interrupt                  */
  ADC_IRQn                      =  24,      /*!< ADC interrupt                      */
  WWDT_IRQn                     =  25,      /*!< Windowed Watchdog interrupt        */
  BOD_IRQn                      =  26,      /*!< Brown-out interrupt                */
  FLASH_IRQn                    =  27,      /*!< Flash/EEPROM interface interrupt   */
  USB_WAKEUP_IRQn               =  30,      /*!< USB wake-up interrupt              */
  IOH_IRQn                      =  31,      /*!< I/O Handler interrupt              */
} IRQn_Type;


/* ================================================================================ */
/* ================      Processor and Core Peripheral Section     ================ */
/* ================================================================================ */

/* --------  Configuration of the Cortex-M4 Processor and Core Peripherals  ------- */
#define __CM0_REV                 0x0000    /*!< Core revision r0p0                              */
#define __MPU_PRESENT             0         /*!< MPU present or not                              */
#define __NVIC_PRIO_BITS          2         /*!< Number of Bits used for Priority Levels         */
#define __Vendor_SysTickConfig    0         /*!< Set to 1 if different SysTick Config is used    */

#include <core_cm0.h>                       /* Processor and core peripherals                    */
#include "system_ARMCM0.h"                  /* System Header                                     */


/* ================================================================================ */
/* ================       Device Specific Peripheral Section       ================ */
/* ================================================================================ */

/* -------------------  Start of section using anonymous unions  ------------------ */
#if defined(__CC_ARM)
  #pragma push
  #pragma anon_unions
#elif defined(__ICCARM__)
  #pragma language=extended
#elif defined(__GNUC__)
  /* anonymous unions are enabled by default */
#elif defined(__TMS470__)
/* anonymous unions are enabled by default */
#elif defined(__TASKING__)
  #pragma warning 586
#else
  #warning Not supported compiler type
#endif



/* ================================================================================ */
/* ================            CPU FPGA System (CPU_SYS)           ================ */
/* ================================================================================ */
typedef struct
{
  __I  uint32_t ID;               /* Offset: 0x000 (R/ )  Board and FPGA Identifier */
  __IO uint32_t MEMCFG;           /* Offset: 0x004 (R/W)  Remap and Alias Memory Control */
  __I  uint32_t SW;               /* Offset: 0x008 (R/ )  Switch States */
  __IO uint32_t LED;              /* Offset: 0x00C (R/W)  LED Output States */
  __I  uint32_t TS;               /* Offset: 0x010 (R/ )  Touchscreen Register */
  __IO uint32_t CTRL1;            /* Offset: 0x014 (R/W)  Misc Control Functions */
       uint32_t RESERVED0[2];
  __IO uint32_t CLKCFG;           /* Offset: 0x020 (R/W)  System Clock Configuration */
  __IO uint32_t WSCFG;            /* Offset: 0x024 (R/W)  Flash Waitstate Configuration */
  __IO uint32_t CPUCFG;           /* Offset: 0x028 (R/W)  Processor Configuration */
       uint32_t RESERVED1[3];
  __IO uint32_t BASE;             /* Offset: 0x038 (R/W)  ROM Table base Address */
  __IO uint32_t ID2;              /* Offset: 0x03C (R/W)  Secondary Identification Register */
} ARM_CPU_SYS_TypeDef;


/* ================================================================================ */
/* ================            DUT FPGA System (DUT_SYS)           ================ */
/* ================================================================================ */
typedef struct
{
  __I  uint32_t ID;               /* Offset: 0x000 (R/ )  Board and FPGA Identifier */
  __IO uint32_t PERCFG;           /* Offset: 0x004 (R/W)  Peripheral Control Signals */
  __I  uint32_t SW;               /* Offset: 0x008 (R/ )  Switch States */
  __IO uint32_t LED;              /* Offset: 0x00C (R/W)  LED Output States */
  __IO uint32_t SEG7;             /* Offset: 0x010 (R/W)  7-segment LED Output States */
  __I  uint32_t CNT25MHz;         /* Offset: 0x014 (R/ )  Freerunning counter incrementing at 25MHz */
  __I  uint32_t CNT100Hz;         /* Offset: 0x018 (R/ )  Freerunning counter incrementing at 100Hz */
} ARM_DUT_SYS_TypeDef;


/* ================================================================================ */
/* ================                   Timer (TIM)                  ================ */
/* ================================================================================ */
typedef struct
{
  __IO uint32_t Timer1Load;       /* Offset: 0x000 (R/W)  Timer 1 Load */
  __I  uint32_t Timer1Value;      /* Offset: 0x004 (R/ )  Timer 1 Counter Current Value */
  __IO uint32_t Timer1Control;    /* Offset: 0x008 (R/W)  Timer 1 Control */
  __O  uint32_t Timer1IntClr;     /* Offset: 0x00C ( /W)  Timer 1 Interrupt Clear */
  __I  uint32_t Timer1RIS;        /* Offset: 0x010 (R/ )  Timer 1 Raw Interrupt Status */
  __I  uint32_t Timer1MIS;        /* Offset: 0x014 (R/ )  Timer 1 Masked Interrupt Status */
  __IO uint32_t Timer1BGLoad;     /* Offset: 0x018 (R/W)  Background Load Register */
       uint32_t RESERVED0[1];
  __IO uint32_t Timer2Load;       /* Offset: 0x020 (R/W)  Timer 2 Load */
  __I  uint32_t Timer2Value;      /* Offset: 0x024 (R/ )  Timer 2 Counter Current Value */
  __IO uint32_t Timer2Control;    /* Offset: 0x028 (R/W)  Timer 2 Control */
  __O  uint32_t Timer2IntClr;     /* Offset: 0x02C ( /W)  Timer 2 Interrupt Clear */
  __I  uint32_t Timer2RIS;        /* Offset: 0x030 (R/ )  Timer 2 Raw Interrupt Status */
  __I  uint32_t Timer2MIS;        /* Offset: 0x034 (R/ )  Timer 2 Masked Interrupt Status */
  __IO uint32_t Timer2BGLoad;     /* Offset: 0x038 (R/W)  Background Load Register */
} ARM_TIM_TypeDef;


/* ================================================================================ */
/* ============== Universal Asyncronous Receiver / Transmitter (UART) ============= */
/* ================================================================================ */
typedef struct
{
  __IO uint32_t DR;               /* Offset: 0x000 (R/W)  Data */ 
  union {
  __I  uint32_t RSR;              /* Offset: 0x000 (R/ )  Receive Status */
  __O  uint32_t ECR;              /* Offset: 0x000 ( /W)  Error Clear */
  };
       uint32_t RESERVED0[4];
  __IO uint32_t FR;               /* Offset: 0x018 (R/W)  Flags */
       uint32_t RESERVED1[1];
  __IO uint32_t ILPR;             /* Offset: 0x020 (R/W)  IrDA Low-power Counter */
  __IO uint32_t IBRD;             /* Offset: 0x024 (R/W)  Interger Baud Rate */
  __IO uint32_t FBRD;             /* Offset: 0x028 (R/W)  Fractional Baud Rate */
  __IO uint32_t LCR_H;            /* Offset: 0x02C (R/W)  Line Control */
  __IO uint32_t CR;               /* Offset: 0x030 (R/W)  Control */
  __IO uint32_t IFLS;             /* Offset: 0x034 (R/W)  Interrupt FIFO Level Select */
  __IO uint32_t IMSC;             /* Offset: 0x038 (R/W)  Interrupt Mask Set / Clear */
  __IO uint32_t RIS;              /* Offset: 0x03C (R/W)  Raw Interrupt Status */
  __IO uint32_t MIS;              /* Offset: 0x040 (R/W)  Masked Interrupt Status */
  __O  uint32_t ICR;              /* Offset: 0x044 ( /W)  Interrupt Clear */
  __IO uint32_t DMACR;            /* Offset: 0x048 (R/W)  DMA Control */
} ARM_UART_TypeDef;


/* --------------------  End of section using anonymous unions  ------------------- */
#if defined(__CC_ARM)
  #pragma pop
#elif defined(__ICCARM__)
  /* leave anonymous unions enabled */
#elif defined(__GNUC__)
  /* anonymous unions are enabled by default */
#elif defined(__TMS470__)
  /* anonymous unions are enabled by default */
#elif defined(__TASKING__)
  #pragma warning restore
#else
  #warning Not supported compiler type
#endif




/* ================================================================================ */
/* ================              Peripheral memory map             ================ */
/* ================================================================================ */
/* --------------------------  CPU FPGA memory map  ------------------------------- */
#define ARM_FLASH_BASE            (0x00000000UL)
#define ARM_RAM_BASE              (0x20000000UL)
#define ARM_RAM_FPGA_BASE         (0x1EFF0000UL)
#define ARM_CPU_CFG_BASE          (0xDFFF0000UL)

#define ARM_CPU_SYS_BASE          (ARM_CPU_CFG_BASE  + 0x00000)
#define ARM_UART3_BASE            (ARM_CPU_CFG_BASE  + 0x05000)

/* --------------------------  DUT FPGA memory map  ------------------------------- */
#define ARM_APB_BASE              (0x40000000UL)
#define ARM_AHB_BASE              (0x4FF00000UL)
#define ARM_DMC_BASE              (0x60000000UL)
#define ARM_SMC_BASE              (0xA0000000UL)

#define ARM_TIM0_BASE             (ARM_APB_BASE      + 0x02000)
#define ARM_TIM2_BASE             (ARM_APB_BASE      + 0x03000)
#define ARM_DUT_SYS_BASE          (ARM_APB_BASE      + 0x04000)
#define ARM_UART0_BASE            (ARM_APB_BASE      + 0x06000)
#define ARM_UART1_BASE            (ARM_APB_BASE      + 0x07000)
#define ARM_UART2_BASE            (ARM_APB_BASE      + 0x08000)
#define ARM_UART4_BASE            (ARM_APB_BASE      + 0x09000)


/* ================================================================================ */
/* ================             Peripheral declaration             ================ */
/* ================================================================================ */
/* --------------------------  CPU FPGA Peripherals  ------------------------------ */
#define ARM_CPU_SYS               ((ARM_CPU_SYS_TypeDef *)  ARM_CPU_SYS_BASE)
#define ARM_UART3                 ((   ARM_UART_TypeDef *)    ARM_UART3_BASE)

/* --------------------------  DUT FPGA Peripherals  ------------------------------ */
#define ARM_DUT_SYS               ((ARM_DUT_SYS_TypeDef *)  ARM_DUT_SYS_BASE)
#define ARM_TIM0                  ((    ARM_TIM_TypeDef *)     ARM_TIM0_BASE)
#define ARM_TIM2                  ((    ARM_TIM_TypeDef *)     ARM_TIM2_BASE)
#define ARM_UART0                 ((   ARM_UART_TypeDef *)    ARM_UART0_BASE)
#define ARM_UART1                 ((   ARM_UART_TypeDef *)    ARM_UART1_BASE)
#define ARM_UART2                 ((   ARM_UART_TypeDef *)    ARM_UART2_BASE)
#define ARM_UART4                 ((   ARM_UART_TypeDef *)    ARM_UART4_BASE)


#ifdef __cplusplus
}
#endif

#endif  /* ARMCM0_H */

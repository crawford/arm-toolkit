######################################################################
#                                                                    #
# LPC11U24_301                                                       #
#                                                                    #
######################################################################

_LPC11U24_301_CPU      := cortex-m0
_LPC11U24_301_CORE     := CORE_M0
_LPC11U24_301_LINKER   := \
    CMSIS/Device/ARM/ARMCM0/LPC11U24_301/Source/GCC/gcc_arm.ld

_LPC11U24_301_SOURCES := \
    arch/cortex-m0/fault.c \
    arch/cortex-m0/lpc11uxx/src/gpio_11xx.c \
    arch/cortex-m0/lpc11uxx/src/adc_11xx.c \
    arch/cortex-m0/lpc11uxx/src/chip_11xx.c \
    arch/cortex-m0/lpc11uxx/src/clock_11xx.c \
    arch/cortex-m0/lpc11uxx/src/gpiogroup_11xx.c \
    arch/cortex-m0/lpc11uxx/src/i2cm_11xx.c \
    arch/cortex-m0/lpc11uxx/src/i2c_11xx.c \
    arch/cortex-m0/lpc11uxx/src/iap_11xx.c \
    arch/cortex-m0/lpc11uxx/src/iocon_11xx.c \
    arch/cortex-m0/lpc11uxx/src/pinint_11xx.c \
    arch/cortex-m0/lpc11uxx/src/pmu_11xx.c \
    arch/cortex-m0/lpc11uxx/src/ring_buffer.c \
    arch/cortex-m0/lpc11uxx/src/ssp_11xx.c \
    arch/cortex-m0/lpc11uxx/src/sysctl_11xx.c \
    arch/cortex-m0/lpc11uxx/src/sysinit_11xx.c \
    arch/cortex-m0/lpc11uxx/src/timer_11xx.c \
    arch/cortex-m0/lpc11uxx/src/uart_11xx.c \
    arch/cortex-m0/lpc11uxx/src/wwdt_11xx.c \
    arch/cortex-m0/lpc11uxx/src/gpio_11xx.c \
    CMSIS/Device/ARM/ARMCM0/LPC11U24_301/Source/GCC/startup_ARMCM0.S \
    drivers/adc/11uxx.c \
    drivers/lamp/lamp.c \
    drivers/timer/timer.c \

_LPC11U24_301_INCLUDES := \
    arch/cortex-m0/lpc11uxx/inc \
    CMSIS/Device/ARM/ARMCM0/LPC11U24_301/Include \
    drivers/adc \
    drivers/lamp \
    drivers/timer \


######################################################################
#                                                                    #
# EFM32LG990F256                                                     #
#                                                                    #
######################################################################

_EFM32LG990F256_CPU    := cortex-m3
_EFM32LG990F256_CORE   := CORE_M3
_EFM32LG990F256_LINKER := \
    CMSIS/Device/ARM/ARMCM3/EFM32LG/Source/GCC/efm32lg.ld

_EFM32LG990F256_SOURCES := \
    CMSIS/Device/ARM/ARMCM3/EFM32LG/Source/GCC/startup_efm32lg.S \
    CMSIS/Device/ARM/ARMCM3/EFM32LG/Source/system_efm32lg.c \

_EFM32LG990F256_INCLUDES := \
    CMSIS/Device/ARM/ARMCM3/EFM32LG/Include \


ifndef _$(MCU)_CPU
    $(error MCU is not supported. See arm-toolkit README)
endif

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
    arch/cortex-m3/emlib/src/em_acmp.c \
    arch/cortex-m3/emlib/src/em_adc.c \
    arch/cortex-m3/emlib/src/em_aes.c \
    arch/cortex-m3/emlib/src/em_assert.c \
    arch/cortex-m3/emlib/src/em_burtc.c \
    arch/cortex-m3/emlib/src/em_cmu.c \
    arch/cortex-m3/emlib/src/em_dac.c \
    arch/cortex-m3/emlib/src/em_dbg.c \
    arch/cortex-m3/emlib/src/em_dma.c \
    arch/cortex-m3/emlib/src/em_ebi.c \
    arch/cortex-m3/emlib/src/em_emu.c \
    arch/cortex-m3/emlib/src/em_gpio.c \
    arch/cortex-m3/emlib/src/em_i2c.c \
    arch/cortex-m3/emlib/src/em_int.c \
    arch/cortex-m3/emlib/src/em_lcd.c \
    arch/cortex-m3/emlib/src/em_lesense.c \
    arch/cortex-m3/emlib/src/em_letimer.c \
    arch/cortex-m3/emlib/src/em_leuart.c \
    arch/cortex-m3/emlib/src/em_mpu.c \
    arch/cortex-m3/emlib/src/em_msc.c \
    arch/cortex-m3/emlib/src/em_opamp.c \
    arch/cortex-m3/emlib/src/em_pcnt.c \
    arch/cortex-m3/emlib/src/em_prs.c \
    arch/cortex-m3/emlib/src/em_rmu.c \
    arch/cortex-m3/emlib/src/em_rtc.c \
    arch/cortex-m3/emlib/src/em_system.c \
    arch/cortex-m3/emlib/src/em_timer.c \
    arch/cortex-m3/emlib/src/em_usart.c \
    arch/cortex-m3/emlib/src/em_vcmp.c \
    arch/cortex-m3/emlib/src/em_wdog.c \
    CMSIS/Device/ARM/ARMCM3/EFM32LG/Source/GCC/startup_efm32lg.S \
    CMSIS/Device/ARM/ARMCM3/EFM32LG/Source/system_efm32lg.c \

_EFM32LG990F256_INCLUDES := \
    arch/cortex-m3/emlib/inc \
    CMSIS/Device/ARM/ARMCM3/EFM32LG/Include \


ifndef _$(MCU)_CPU
    $(error MCU is not supported. See arm-toolkit README)
endif

######################################################################
#                                                                    #
# User Defined Variables                                             #
#                                                                    #
#   TOOLKIT_PATH - path to the arm-toolkit directory                 #
#                  (required)                                        #
#   SOURCES      - list of user source files                         #
#                  (required)                                        #
#   OUTPUT       - name of the build artifacts (excluding suffix)    #
#                  (default: out)                                    #
#   BUILD_DIR    - path to the build directory                       #
#                  (default: build)                                  #
#   CROSS_PREFIX - prefix for all of the build tools                 #
#                  (default: arm-none-eabi-)                         #
#   SFLAGS       - flags for the assembler                           #
#   CFLAGS       - flags for the C compiler                          #
#   LDFLAGS      - flags for the linker                              #
#   VERBOSE      - unless equal to 1, silence the build output       #
#                                                                    #
######################################################################

ifndef TOOLKIT_PATH
    $(error TOOLKIT_PATH must be defined. See arm-toolkit README)
endif

ifndef SOURCES
    $(error SOURCES must be defined. See arm-toolkit README)
endif

ifneq ($(VERBOSE),1)
    Q := @
endif

OUTPUT       ?= out
BUILD_DIR    ?= build
CROSS_PREFIX ?= arm-none-eabi-

SFLAGS ?= \
    -mcpu=cortex-m0 \
    -mthumb \
    --gstabs+ \

CFLAGS ?= \
    -mcpu=cortex-m0 \
    -mthumb \
    -Wall \
    -ffunction-sections \
    -fdata-sections \
    -ggdb3 \
    -Og \
    -std=c99 \
    -DCORE_M0 \

LDFLAGS ?= \
    -mcpu=cortex-m0 \
    -mthumb \
    -T$(TOOLKIT_PATH)/src/CMSIS/Device/ARM/ARMCM0/LPC11U24_301/Source/GCC/gcc_arm.ld \
    -Wl,-gc-sections \

######################################################################
#                                                                    #
# Internal Variables                                                 #
#                                                                    #
######################################################################

CC := $(CROSS_PREFIX)gcc
AS := $(CROSS_PREFIX)as
LD := $(CROSS_PREFIX)gcc
OC := $(CROSS_PREFIX)objcopy
SZ := $(CROSS_PREFIX)size

ELF = $(BUILD_DIR)/$(OUTPUT).elf
BIN := $(BUILD_DIR)/$(OUTPUT).bin

_SOURCES := \
    fault.c \
    drivers/adc/11uxx.c \
    drivers/lamp/lamp.c \
    drivers/timer/timer.c \
    arch/cortex-m0/lpc11uxx/src/gpio_11xx.c \
    arch/cortex-m0/lpc11uxx/src/adc_11xx.c \
    arch/cortex-m0/lpc11uxx/src/chip_11xx.c \
    arch/cortex-m0/lpc11uxx/src/clock_11xx.c \
    arch/cortex-m0/lpc11uxx/src/gpiogroup_11xx.c \
    arch/cortex-m0/lpc11uxx/src/i2cm_11xx.c \
    arch/cortex-m0/lpc11uxx/src/i2c_11xx.c \
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
    hardfault.S \
    CMSIS/Device/ARM/ARMCM0/LPC11U24_301/Source/GCC/startup_ARMCM0.S \

_INCLUDES := \
    . \
    drivers/adc \
    drivers/lamp \
    drivers/timer \
    CMSIS/Device/ARM/ARMCM0/LPC11U24_301/Include \
    CMSIS/CMSIS/Include \
    arch/cortex-m0/lpc11uxx/inc \

CFLAGS += $(_INCLUDES:%=-I$(TOOLKIT_PATH)/src/%)

_OBJS := $(SOURCES) $(_SOURCES:%=$(TOOLKIT_PATH)/src/%)
_OBJS := $(_OBJS:%.c=$(BUILD_DIR)/%.o)
_OBJS := $(_OBJS:%.S=$(BUILD_DIR)/%.o)

.PHONY: all clean size

all: $(BIN)

size: $(ELF)
	$(Q)$(SZ) $<

clean:
	$(Q)rm -f $(_OBJS)
	$(Q)rm -f $(ELF)
	$(Q)rm -f $(BIN)
	$(Q)[ -d $(BUILD_DIR) ] && \
	    find $(BUILD_DIR) -depth -type d -print0 | \
	    xargs -0 rmdir --ignore-fail-on-non-empty || \
	    exit 0

$(BIN): $(ELF)
	$(Q)mkdir -p "`dirname $@`"
	$(Q)echo " Copy    " $<
	$(Q)$(OC) --output-target binary $< $@

$(ELF): $(_OBJS)
	$(Q)mkdir -p "`dirname $@`"
	$(Q)echo " LD      " $<
	$(Q)$(LD) $(LDFLAGS) $^ --output $@

$(BUILD_DIR)/%.o: %.c
	$(Q)mkdir -p "`dirname $@`"
	$(Q)echo " CC      " $<
	$(Q)$(CC) $(CFLAGS) -c $< --output $@

$(BUILD_DIR)/%.o: %.S
	$(Q)mkdir -p "`dirname $@`"
	$(Q)echo " AS      " $<
	$(Q)$(AS) $(SFLAGS) $< -o $@

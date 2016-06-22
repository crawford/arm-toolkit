######################################################################
#                                                                    #
# User Defined Variables                                             #
#                                                                    #
#   MCU          -                                                   #
#                  (required)                                        #
#   TOOLKIT_PATH - path to the arm-toolkit directory                 #
#                  (required)                                        #
#   SOURCES      - list of user source files                         #
#                  (required)                                        #
#   INCLUDES     - list of user include directories                  #
#                  (default: )                                       #
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

ifndef MCU
    $(error MCU must be defined. See arm-toolkit README)
endif

ifndef SOURCES
    $(error SOURCES must be defined. See arm-toolkit README)
endif

include $(TOOLKIT_PATH)/mcus.mk

ifneq ($(VERBOSE),1)
    Q := @
endif

OUTPUT       ?= out
BUILD_DIR    ?= build
CROSS_PREFIX ?= arm-none-eabi-

SFLAGS ?= \
    -mcpu=$(_$(MCU)_CPU) \
    -mthumb \
    --gstabs+ \

CFLAGS ?= \
    -mcpu=$(_$(MCU)_CPU) \
    -mthumb \
    -Wall \
    -ffunction-sections \
    -fdata-sections \
    -ggdb3 \
    -Os \
    -flto \
    -std=c99 \
    -D$(_$(MCU)_CORE) \
    -D$(MCU) \

LDFLAGS ?= \
    -mcpu=$(_$(MCU)_CPU) \
    -mthumb \
    -T$(TOOLKIT_PATH)/src/$(_$(MCU)_LINKER) \
    -Wl,-gc-sections \
    -flto \

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
    hardfault.S \
    $(_$(MCU)_SOURCES) \

_INCLUDES := \
    . \
    CMSIS/CMSIS/Include \
    $(_$(MCU)_INCLUDES) \


CFLAGS += $(INCLUDES:%=-I%) $(_INCLUDES:%=-I$(TOOLKIT_PATH)/src/%)

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

/*
 * Copyright 2015 Alex Crawford
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 *     Unless required by applicable law or agreed to in writing, software
 *     distributed under the License is distributed on an "AS IS" BASIS,
 *     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *     See the License for the specific language governing permissions and
 *     limitations under the License.
 */

#ifndef _IAP_11XX_H_
#define _IAP_11XX_H_

#include <stdint.h>
#include "chip.h"

#define IAP_CMD_PREPARE_WRITE          50
#define IAP_CMD_COPY_TO_FLASH          51
#define IAP_CMD_ERASE_SECTORS          52
#define IAP_CMD_BLANK_CHECK_SECTORS    53
#define IAP_CMD_READ_PART_ID           54
#define IAP_CMD_READ_BOOT_VERSION      55
#define IAP_CMD_COMPARE                56
#define IAP_CMD_REINVOKE_ISP           57
#define IAP_CMD_READ_UID               58
#define IAP_CMD_ERASE_PAGES            59
#define IAP_CMD_EEPROM_WRITE           61
#define IAP_CMD_EEPROM_READ            62

typedef enum iap_result {
	IAP_COMMAND_SUCCESS                         = 0,
	IAP_INVALID_COMMAND                         = 1,
	IAP_SOURCE_ADDRESS_ERROR                    = 2,
	IAP_DESTINATION_ADDRESS_ERROR               = 3,
	IAP_SOURCE_ADDRESS_NOT_MAPPED               = 4,
	IAP_DESTINATION_ADDRESS_NOT_MAPPED          = 5,
	IAP_COUNT_ERROR                             = 6,
	IAP_INVALID_SECTOR                          = 7,
	IAP_SECTOR_NOT_BLANK                        = 8,
	IAP_SECTOR_NOT_PREPARED_FOR_WRITE_OPERATION = 9,
	IAP_COMPARE_ERROR                           = 10,
	IAP_BUSY                                    = 11,
} iap_result_e;

typedef void (*iap_request_t)(uint32_t *, uint32_t *);

static inline void iap_request(uint32_t *cmd_param,
                               uint32_t *status_result)
{
	((iap_request_t)IAP_ENTRY_LOCATION)(cmd_param, status_result);
}

// This command must be executed before executing "Copy RAM to flash" or "Erase
// Sector(s)" command. Successful execution of the "Copy RAM to flash" or
// "Erase Sector(s)" command causes relevant sectors to be protected again. The
// boot sector can not be prepared by this command. To prepare a single sector
// use the same "Start" and "End" sector numbers.
//
// Returns:
//   IAP_COMMAND_SUCCESS
//   IAP_INVALID_SECTOR
//   IAP_BUSY
static inline iap_result_e iap_prepare_write(uint32_t start_sec,
                                             uint32_t end_sec);

//  This command is used to program the flash memory. The affected sectors
//  should be prepared first by calling "Prepare Sector for Write Operation"
//  command. The affected sectors are automatically protected again once the
//  copy command is successfully executed. The boot sector can not be written
//  by this command.
//
//  Returns:
//    IAP_COMMAND_SUCCESS
//    IAP_SOURCE_ADDRESS_ERROR
//    IAP_DESTINATION_ADDRESS_ERROR
//    IAP_SOURCE_ADDRESS_NOT_MAPPED
//    IAP_DESTINATION_ADDRESS_NOT_MAPPED
//    IAP_COUNT_ERROR
//    IAP_SECTOR_NOT_PREPARED_FOR_WRITE_OPERATION
//    IAP_BUSY
static inline iap_result_e iap_copy_to_flash(uint32_t dst_addr,
                                             uint32_t src_addr,
                                             uint32_t count,
                                             uint32_t freq);

// This command is used to erase a sector or multiple sectors of on-chip flash
// memory. The boot sector can not be erased by this command. To erase a single
// sector use the same "Start" and "End" sector numbers.
//
// Returns:
//   IAP_COMMAND_SUCCESS
//   IAP_INVALID_SECTOR
//   IAP_SECTOR_NOT_PREPARED_FOR_WRITE_OPERATION
//   IAP_BUSY
static inline iap_result_e iap_erase_sectors(uint32_t start_sec,
                                             uint32_t end_sec,
                                             uint32_t freq);

// This command is used to blank check a sector or multiple sectors of on-chip
// flash memory. To blank check a single sector use the same "Start" and "End"
// sector numbers.
//
// Results:
//   addr: Offset of the first non blank word location if the return code is
//         IAP_SECTOR_NOT_BLANK.
//   value: Contents of non blank word location.
//
// Returns:
//   IAP_COMMAND_SUCCESS
//   IAP_INVALID_SECTOR
//   IAP_SECTOR_NOT_BLANK
//   IAP_BUSY
static inline iap_result_e iap_blank_check_sectors(uint32_t start_sec,
                                                   uint32_t end_sec,
                                                   uint32_t *addr,
                                                   uint32_t *value);

// This command is used to read the part identification number.
//
// Results:
//   id: Part Identification Number
//
// Returns:
//   IAP_COMMAND_SUCCESS
static inline iap_result_e iap_read_part_id(uint32_t *id);

// This command is used to read the boot code version number.
//
// Results:
//   major: Major version number
//   minor: Minor version number
//
// Returns:
//   IAP_COMMAND_SUCCESS
static inline iap_result_e iap_read_boot_version(uint8_t *major,
                                                 uint8_t *minor);

// This command is used to compare the memory contents at two locations.  The
// result may not be correct when the source or destination includes any of the
// first 512 bytes starting from address zero. The first 512 bytes can be
// re-mapped to RAM.
//
// Results:
//   addr: Offset of the first mismatch if the return code is IAP_COMPARE_ERROR
//
// Returns:
//   IAP_COMMAND_SUCCESS
//   IAP_COUNT_ERROR
//   IAP_COMPARE_ERROR
//   ?? ADDR_ERROR ??
//   ?? ADDR_NOT_MAPPED ??
static inline iap_result_e iap_compare(uint32_t dst_addr,
                                       uint32_t src_addr,
                                       uint32_t count,
                                       uint32_t *addr);

// This command is used to invoke the bootloader in ISP mode. It maps boot
// vectors, sets PCLK = CCLK, configures UART pins RXD and TXD, resets
// counter/timer CT32B1 and resets the U0FDR. This command may be used when a
// valid user program is present in the internal flash memory and the PIO0_1
// pin is not accessible to force the ISP mode.
static inline void __attribute__ ((noreturn)) iap_reinvoke_isp();

// This command is used to read the unique ID.
//
// Results:
//   uid: The 128-bit unique identifier
//
// Returns:
//   IAP_COMMAND_SUCCESS
static inline iap_result_e iap_read_uid(uint32_t uid[static 4]);

// This command is used to erase a page or multiple pages of on-chip flash
// memory. To erase a single page use the same "start" and "end" page numbers.
//
// Remark:
//   This command is only implemented on LPC11U3X chips.
//
// Returns:
//   IAP_COMMAND_SUCCESS
//   IAP_INVALID_SECTOR
//   IAP_SECTOR_NOT_PREPARED_FOR_WRITE_OPERATION
//   IAP_BUSY
static inline iap_result_e iap_erase_pages(uint32_t start_page,
                                           uint32_t end_page,
                                           uint32_t freq);

// Data is copied from the RAM address to the EEPROM address.
//
// Remark:
//   The top 64 bytes of the 4 kB EEPROM memory are reserved and cannot be
//   written to. The entire EEPROM is writable for smaller EEPROM sizes.
//
// Returns:
//   IAP_COMMAND_SUCCESS
//   IAP_SOURCE_ADDRESS_NOT_MAPPED
//   IAP_DESTINATION_ADDRESS_NOT_MAPPED
static inline iap_result_e iap_write_eeprom(uint32_t eeprom_addr,
                                            uint8_t *const data,
                                            uint32_t count,
                                            uint32_t freq);

// Data is copied from the EEPROM address to the RAM address.
//
// Results:
//   data: The EEPROM data that was read
//
// Returns:
//   IAP_COMMAND_SUCCESS
//   IAP_SOURCE_ADDRESS_NOT_MAPPED
//   IAP_DESTINATION_ADDRESS_NOT_MAPPED
static inline iap_result_e iap_read_eeprom(uint32_t eeprom_addr,
                                           uint8_t *data,
                                           uint32_t count,
                                           uint32_t freq);

#endif // _IAP_11XX_H_


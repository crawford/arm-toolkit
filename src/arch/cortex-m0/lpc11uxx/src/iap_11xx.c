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

#include <string.h>
#include "iap_11xx.h"

iap_result_e iap_prepare_write(uint32_t start_sec,
                               uint32_t end_sec)
{
	uint32_t cmd[] = { IAP_CMD_PREPARE_WRITE, start_sec, end_sec };
	uint32_t result;
	iap_request(cmd, &result);
	return result;
}

iap_result_e iap_copy_to_flash(uint32_t dst_addr,
                               uint32_t src_addr,
                               uint32_t count,
                               uint32_t freq)
{
	uint32_t cmd[] = { IAP_CMD_COPY_TO_FLASH, dst_addr, src_addr, count, freq };
	uint32_t result;
	iap_request(cmd, &result);
	return result;
}

iap_result_e iap_erase_sectors(uint32_t start_sec,
                               uint32_t end_sec,
                               uint32_t freq)
{
	uint32_t cmd[] = { IAP_CMD_PREPARE_WRITE, start_sec, end_sec, freq };
	uint32_t result;
	iap_request(cmd, &result);
	return result;
}

iap_result_e iap_blank_check_sectors(uint32_t start_sec,
                                     uint32_t end_sec,
                                     uint32_t *addr,
                                     uint32_t *value)
{
	uint32_t cmd[] = { IAP_CMD_BLANK_CHECK_SECTORS, start_sec, end_sec };
	uint32_t result[3];
	iap_request(cmd, result);
	*addr = result[1];
	*value = result[2];
	return result[0];
}

iap_result_e iap_read_part_id(uint32_t *id)
{
	uint32_t cmd[] = { IAP_CMD_READ_PART_ID };
	uint32_t result[2];
	iap_request(cmd, result);
	*id = result[1];
	return result[0];
}

iap_result_e iap_read_boot_version(uint8_t *major,
                                   uint8_t *minor)
{
	uint32_t cmd[] = { IAP_CMD_READ_BOOT_VERSION };
	uint32_t result[2];
	iap_request(cmd, result);
	*major = (result[1] >> 8);
	*minor = (result[1] >> 0);
	return result[0];
}

iap_result_e iap_compare(uint32_t dst_addr,
                         uint32_t src_addr,
                         uint32_t count,
                         uint32_t *addr)
{
	uint32_t cmd[] = { IAP_CMD_COMPARE, dst_addr, src_addr, count };
	uint32_t result[2];
	iap_request(cmd, result);
	*addr = result[1];
	return result[0];
}

void __attribute__ ((noreturn)) iap_reinvoke_isp()
{
	uint32_t cmd[] = { IAP_CMD_REINVOKE_ISP };
	iap_request(cmd, NULL);
	__builtin_unreachable();
}

iap_result_e iap_read_uid(uint32_t uid[static 4])
{
	uint32_t cmd[] = { IAP_CMD_READ_UID };
	uint32_t result[5];
	iap_request(cmd, result);
	memcpy(uid, result + 1, sizeof(*uid) * 4);
	return result[0];
}

iap_result_e iap_erase_pages(uint32_t start_page,
                             uint32_t end_page,
                             uint32_t freq)
{
	uint32_t cmd[] = { IAP_CMD_ERASE_PAGES, start_page, end_page, freq };
	uint32_t result;
	iap_request(cmd, &result);
	return result;
}

iap_result_e iap_write_eeprom(uint32_t eeprom_addr,
                              uint8_t *const data,
                              uint32_t count,
                              uint32_t freq)
{
	uint32_t cmd[] = { IAP_CMD_EEPROM_WRITE, eeprom_addr, (uintptr_t)data, count, freq };
	uint32_t result;
	iap_request(cmd, &result);
	return result;
}

iap_result_e iap_read_eeprom(uint32_t eeprom_addr,
                             uint8_t *data,
                             uint32_t count,
                             uint32_t freq)
{
	uint32_t cmd[] = { IAP_CMD_EEPROM_READ, eeprom_addr, (uintptr_t)data, count, freq };
	uint32_t result;
	iap_request(cmd, &result);
	return result;
}


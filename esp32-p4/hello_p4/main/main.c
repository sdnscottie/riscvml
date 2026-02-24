#include <stdio.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "esp_chip_info.h"
#include "esp_flash.h"
#include "esp_system.h"

void app_main(void)
{
    printf("Hello Scott, from ESP32-P4!\n");

    esp_chip_info_t chip_info;
    esp_chip_info(&chip_info);

    printf("Chip: %s, %d core(s), revision %d\n",
           CONFIG_IDF_TARGET,
           chip_info.cores,
           chip_info.revision);

    printf("Features:%s%s%s\n",
           (chip_info.features & CHIP_FEATURE_WIFI_BGN) ? " WiFi" : "",
           (chip_info.features & CHIP_FEATURE_BLE) ? " BLE" : "",
           (chip_info.features & CHIP_FEATURE_IEEE802154) ? " 802.15.4" : "");

    uint32_t flash_size;
    if (esp_flash_get_size(NULL, &flash_size) == ESP_OK) {
        printf("Flash size: %luMB (%s)\n",
               (unsigned long)(flash_size / (1024 * 1024)),
               (chip_info.features & CHIP_FEATURE_EMB_FLASH) ? "embedded" : "external");
    }

    printf("Free heap: %lu bytes\n", (unsigned long)esp_get_free_heap_size());

    while (1) {
        vTaskDelay(pdMS_TO_TICKS(1000));
    }
}
